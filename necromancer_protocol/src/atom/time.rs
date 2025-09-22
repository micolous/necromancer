//! Time and timecode atoms
//!
//! ## Unimplemented atoms
//!
//! FourCC | Atom name | Length
//! ------ | --------- | ------
//! `TcLk` | `TimecodeLocked` | 0xc
//! `TcSt` | `TimecodeStatus` | 0xc
//! `DSTV` | `DisplayClockTime` | 0x10
//! `DCSC` | `ControlDisplayClock` | 0xc

use binrw::binrw;
use chrono::{DateTime, FixedOffset, Offset, TimeZone};
use std::time::Duration;

use crate::{error::Error, Result};

/// `Time`: Timecode (clock) command/event.
///
/// Used as a:
///
/// * *command*, to set the current timecode on a switcher
/// * *event*, for the switcher to report the time of its last state change,
///   approximately every 0.5 seconds
///
/// The switcher can run in either "time of day" or "free running" mode, so this
/// may or may not represent wall-clock time.
///
/// The maximum `frame` value depends on the framerate of the output.
///
/// **See also:** [`SetTimeOfDay`], [`SetTimecodeConfig`]
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub frame: u8,

    #[brw(pad_size_to = 4)]
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub drop_frame: bool,
}

impl Time {
    fn check(&self) -> Result<()> {
        if self.minute > 60 || self.second > 60 {
            error!("timecode out of expected range: {self:?}");
            return Err(Error::ParameterOutOfRange);
        }

        Ok(())
    }

    /// Convert a [`Duration`][] to the nearest timecode, based on an integer frame rate.
    ///
    /// This does not support drop-frames nor non-integer framerates.
    pub fn to_duration(&self, framerate: u8) -> Result<Duration> {
        if self.frame > framerate {
            error!("frame > framerate {framerate} for {self:?}");
            return Err(Error::ParameterOutOfRange);
        }
        if self.drop_frame {
            return Err(Error::DropFrame);
        }
        self.check()?;

        let secs = (((self.hour as u64 * 60) + self.minute as u64) * 60) + self.second as u64;
        let micros = (secs * 1_000_000) + ((self.frame as u64 * 1_000_000) / framerate as u64);
        Ok(Duration::from_micros(micros))
    }

    /// Convert a [`Duration`][] to the nearest timecode, based on an integer frame rate.
    ///
    /// This does not support drop-frames nor non-integer framerates.
    pub fn from_duration(duration: &Duration, framerate: u8) -> Result<Self> {
        let seconds = duration.as_secs();
        if seconds >= 921600 {
            error!("duration {duration:?} > 921600 seconds");
            return Err(Error::ParameterOutOfRange);
        }

        Ok(Time {
            hour: (seconds / 3600) as u8,
            minute: ((seconds % 3600) / 60) as u8,
            second: (seconds % 60) as u8,
            frame: (duration.subsec_micros() / (1_000_000 / (framerate as u32))) as u8,
            drop_frame: false,
        })
    }
}

/// `TCCc`: Timecode configuration change.
///
/// See also: [`SetTimecodeConfig`][]
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub struct TimecodeConfig(#[brw(pad_after = 3)] pub TimeMode);

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum TimeMode {
    FreeRun = 0,
    TimeOfDay = 1,
}

/// `CTCC`: Change timecode config
///
/// See also: [`TimecodeConfig`][]
#[binrw]
#[derive(Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub struct SetTimecodeConfig(#[brw(pad_after = 3)] pub TimeMode);

/// `TiRq`: request current timecode
///
/// This causes the switcher to send a [Time] command to all connected clients.
#[binrw]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimecodeRequest {}

/// `SToD`: Set time of day
///
/// Sets the device's internal clock and timezone.
///
/// **See also:** [Time], [SetTimecodeConfig]
///
/// ## Packet format
///
/// * `i32`: [current time][Self::time_sec]
/// * `i32`: [UTC offset][Self::utc_offset]
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetTimeOfDay {
    /// Current time, in [seconds since UNIX epoch][0] (ignoring leap seconds).
    ///
    /// [0]: https://en.wikipedia.org/wiki/Unix_time
    pub time_sec: i32,

    /// Number of minutes the local timezone is _behind_ UTC.
    ///
    /// eg: `-600` = UTC+10:00
    pub utc_offset: i32,
}

impl SetTimeOfDay {
    /// Converts the [SetTimeOfDay] command into a [FixedOffset].
    #[inline]
    pub fn to_offset(&self) -> Option<FixedOffset> {
        FixedOffset::west_opt(self.utc_offset * 60)
    }

    /// Converts the [SetTimeOfDay] command into a [DateTime] with
    /// [FixedOffset].
    pub fn to_datetime(&self) -> Option<DateTime<FixedOffset>> {
        self.to_offset()?
            .timestamp_opt(self.time_sec.into(), 0)
            .single()
    }

    /// Creates a [SetTimeOfDay] from a `chrono` [DateTime].
    ///
    /// Returns `None` if the [DateTime] is [before the UNIX epoch][0], or
    /// [on/after 2038-01-19 03:14:07 UTC][1].
    ///
    /// Sub-minute UTC offsets are silently discarded.
    ///
    /// [0]: https://en.wikipedia.org/wiki/Unix_time
    /// [1]: https://en.wikipedia.org/wiki/Year_2038_problem
    pub fn from_datetime(when: &DateTime<impl TimeZone>) -> Option<Self> {
        // trace!("when.timestamp = {}", when.timestamp());
        let time_sec = when.timestamp().try_into().ok()?;
        let utc_offset = when.offset().fix().utc_minus_local();

        Some(Self {
            time_sec,
            utc_offset: utc_offset / 60,
        })
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use binrw::{BinRead, BinWrite};
    use chrono::Utc;

    use crate::{
        atom::{Atom, Payload},
        packet::{AtemPacket, AtemPacketFlags},
    };

    use super::*;

    #[test]
    fn time() -> Result<()> {
        // response from TiRq command
        let expected = Time {
            hour: 16,
            minute: 31,
            second: 22,
            frame: 25,
            drop_frame: false,
        };
        let cmd = hex::decode("881c80072559000000003a4c0010000054696d65101f161900000000")?;
        let pkt = AtemPacket::read(&mut Cursor::new(&cmd))?;
        let payload = pkt.atoms().expect("wrong payload type");

        assert_eq!(1, payload.len());
        let Payload::Time(time) = payload[0].payload.clone() else {
            panic!("wrong command type");
        };

        assert_eq!(expected, time);
        assert_eq!(Duration::from_millis(59482_500), time.to_duration(50)?);
        assert!(matches!(
            time.to_duration(24),
            Err(Error::ParameterOutOfRange)
        ));
        assert_eq!(
            time,
            Time::from_duration(&Duration::from_millis(59482_500), 50)?
        );
        assert_eq!(
            Time {
                hour: 16,
                minute: 31,
                second: 22,
                frame: 12,
                drop_frame: false,
            },
            Time::from_duration(&Duration::from_millis(59482_500), 25)?
        );

        let o = AtemPacket::new_atoms(
            AtemPacketFlags::new().with_ack(true).with_response(true),
            0x8007,
            0x2559,
            0x0,
            0x3a4c,
            vec![Atom::new(expected)],
        );
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn tccc() -> Result<()> {
        // modified to remove uninitialised memory
        let cmd = hex::decode("000c00005443436300000000")?;
        let ct = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::TimecodeConfig(mode) = ct.payload else {
            panic!("wrong command type");
        };

        assert_eq!(TimeMode::FreeRun, mode.0);

        let o = Atom::new(TimecodeConfig(TimeMode::FreeRun));
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());

        let cmd = hex::decode("000c00005443436301000000")?;
        let ct = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::TimecodeConfig(mode) = ct.payload else {
            panic!("wrong command type");
        };

        assert_eq!(TimeMode::TimeOfDay, mode.0);

        let o = Atom::new(TimecodeConfig(TimeMode::TimeOfDay));
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());

        Ok(())
    }

    #[test]
    fn ctcc() -> Result<()> {
        // modified to remove uninitialised memory
        let cmd = hex::decode("000c00004354434300000000")?;
        let ct = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::SetTimecodeConfig(mode) = ct.payload else {
            panic!("wrong command type");
        };

        assert_eq!(TimeMode::FreeRun, mode.0);

        let o = Atom::new(SetTimecodeConfig(TimeMode::FreeRun));
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());

        let cmd = hex::decode("000c00004354434301000000")?;
        let ct = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::SetTimecodeConfig(mode) = ct.payload else {
            panic!("wrong command type");
        };

        assert_eq!(TimeMode::TimeOfDay, mode.0);

        let o = Atom::new(SetTimecodeConfig(TimeMode::TimeOfDay));
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());

        Ok(())
    }

    #[test]
    fn tirq() -> Result<()> {
        // Modified to remove initialised memory
        let cmd = hex::decode("0008000054695271")?;
        let ct = Atom::read(&mut Cursor::new(&cmd))?;

        assert!(matches!(ct.payload, Payload::TimecodeRequest(_)));

        let o = Atom::new(TimecodeRequest {});
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn stod() -> Result<()> {
        let expected = SetTimeOfDay {
            time_sec: 1695092437,
            utc_offset: -600,
        };
        // Modified to remove uninitialised memory
        let cmd = hex::decode("0010000053546f4465090ed5fffffda8")?;
        let ct = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::SetTimeOfDay(stod) = ct.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, stod);

        // Check DateTime conversions
        let t = stod.to_datetime().expect("could not parse time");
        assert_eq!("2023-09-19T13:00:37+10:00", t.to_rfc3339());
        assert_eq!(
            expected,
            SetTimeOfDay::from_datetime(
                &"2023-09-19T13:00:37+10:00"
                    .parse::<DateTime<FixedOffset>>()
                    .unwrap()
            )
            .unwrap(),
        );

        // Check serialisation
        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn stod_edge_cases() -> Result<()> {
        let expected = SetTimeOfDay {
            time_sec: 1695092437,
            utc_offset: -600,
        };

        // Sub-minute offsets should be rounded to the nearest minute
        assert_eq!(
            Some(expected),
            SetTimeOfDay::from_datetime(
                &FixedOffset::east_opt(36001)
                    .expect("creating 36001 second TZ offset")
                    .with_ymd_and_hms(2023, 9, 19, 13, 00, 38)
                    .single()
                    .expect("creating timestamp")
            )
        );

        // Pre-epoch times should fail conversion
        assert_eq!(
            None,
            SetTimeOfDay::from_datetime(
                &Utc.with_ymd_and_hms(1900, 1, 1, 0, 0, 0)
                    .single()
                    .expect("creating old timestamp")
            )
        );

        // Post-overflow times should fail conversion
        assert_eq!(
            None,
            SetTimeOfDay::from_datetime(
                &Utc.with_ymd_and_hms(2100, 1, 1, 0, 0, 0)
                    .single()
                    .expect("creating future timestamp")
            )
        );

        Ok(())
    }
}
