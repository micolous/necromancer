//! # Disk recording; 4/10 atoms
//!
//! ## Unimplemented atoms
//!
//! Seen atoms
//!
//! FourCC | Atom name | Length
//! ------ | --------- | ------
//! `RMRD` | `RecordToMediaRecordingDuration` | 0x10
//! `RTMD` | `RecordToMediaDisk` | 0x54
//!
//! Not seen
//!
//! FourCC | Atom name | Length
//! ------ | --------- | ------
//! `CRMS` | `ChangeRecordToMediaSetup` | 0x98
//! `ISOi` | `RecordAllISOInputs` | 0xc
//! `RMSp` | `RecordToMediaSwitchDisk` | 0x8
//! `RMSu` | `RecordToMediaSetup` | 0x94

use crate::atom::Time;
use binrw::{binrw, BinRead, BinWrite};
use modular_bitfield::{bitfield, prelude::B7};
use std::ops::{Deref, DerefMut};

/// `RcTM`: ReCord To Media
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordToMedia {
    #[brw(pad_size_to = 4)]
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub start: bool,
}

/// `RTMS`: Record To Media Status
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordToMediaStatus {
    #[brw(pad_size_to = 4)]
    pub status: RecordStatus,

    pub total_recording_time_available: u32,
}

/// Recording status
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Specifier, BinRead, BinWrite, Debug, Default, PartialEq, Eq, Clone, Copy)]
#[br(map = From::<u16>::from)]
#[bw(map = |&x| Into::<u16>::into(x))]
pub struct RecordStatus {
    pub recording: bool,
    pub has_media: bool,
    pub media_full: bool,
    pub media_error: bool,
    pub media_unformatted: bool,
    pub dropping_frames: bool,
    #[skip]
    __: bool,

    pub stopping: bool,
    #[skip]
    __: B7,

    pub unknown_error: bool,
}

/// `RMDR`: Record to Media Duration Request (`RecordToMediaDurationRequest`)
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordToMediaDurationRequest {}

/// `RTMR`: Record to Media Recording timecode (`RecordToMediaRecordingTimecode`)
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordToMediaRecordingTimecode(pub Time);

impl From<Time> for RecordToMediaRecordingTimecode {
    fn from(value: Time) -> Self {
        Self(value)
    }
}

impl From<RecordToMediaRecordingTimecode> for Time {
    fn from(value: RecordToMediaRecordingTimecode) -> Self {
        value.0
    }
}

impl Deref for RecordToMediaRecordingTimecode {
    type Target = Time;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RecordToMediaRecordingTimecode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{atom::Atom, Result};
    use binrw::BinRead;
    use std::{io::Cursor, time::Duration};

    #[test]
    fn rctm() -> Result<()> {
        // Start recording
        let cmd = hex::decode("000c00005263544d01000000")?;
        let cmd = Atom::read(&mut Cursor::new(&cmd))?;

        let expected = Atom::new(RecordToMedia { start: true });
        assert_eq!(expected, cmd);

        // Stop recording
        let cmd = hex::decode("000c00005263544d00000000")?;
        let cmd = Atom::read(&mut Cursor::new(&cmd))?;

        let expected = Atom::new(RecordToMedia { start: false });
        assert_eq!(expected, cmd);

        Ok(())
    }

    #[test]
    fn rtms() -> Result<()> {
        // Recording running
        let cmd = hex::decode("0010000052544d5300030000000a1a5a")?;
        let cmd = Atom::read(&mut Cursor::new(&cmd))?;

        let expected = Atom::new(RecordToMediaStatus {
            status: RecordStatus::new()
                .with_has_media(true)
                .with_recording(true),
            total_recording_time_available: 0x0a1a5a,
        });
        assert_eq!(expected, cmd);

        // Recording stopping
        let cmd = hex::decode("0010ffff52544d5300830001000a1a59")?;
        let cmd = Atom::read(&mut Cursor::new(&cmd))?;

        let expected = Atom::new(RecordToMediaStatus {
            status: RecordStatus::new()
                .with_stopping(true)
                .with_has_media(true)
                .with_recording(true),
            total_recording_time_available: 0x0a1a59,
        });
        assert_eq!(expected, cmd);

        // Recording stopped
        let cmd = hex::decode("0010000052544d5300020000000a1a59")?;
        let cmd = Atom::read(&mut Cursor::new(&cmd))?;

        let expected = Atom::new(RecordToMediaStatus {
            status: RecordStatus::new().with_has_media(true),
            total_recording_time_available: 0x0a1a59,
        });
        assert_eq!(expected, cmd);

        Ok(())
    }

    #[test]
    fn rmdr() -> Result<()> {
        let cmd = hex::decode("00080000524d4452")?;
        let cmd = Atom::read(&mut Cursor::new(&cmd))?;

        let expected = Atom::new(RecordToMediaDurationRequest {});
        assert_eq!(expected, cmd);

        Ok(())
    }

    #[test]
    fn rtmr() -> Result<()> {
        let cmd = hex::decode("0010000052544d520000011700010000")?;
        let cmd = Atom::read(&mut Cursor::new(&cmd))?;

        let timecode = RecordToMediaRecordingTimecode(Time {
            hour: 0,
            minute: 0,
            second: 1,
            frame: 23,
            drop_frame: false,
        });

        let expected = Atom::new(timecode.clone());
        assert_eq!(expected, cmd);

        let t = timecode.to_duration(50)?;
        assert_eq!(Duration::from_millis(1_000 + (23_000 / 50)), t);

        Ok(())
    }
}
