//! Video commands and structures

use std::fmt::Display;

use binrw::{binrw, BinRead, BinWrite};
#[cfg(feature = "clap")]
use clap::ValueEnum;
use modular_bitfield::{
    bitfield,
    specifiers::{B4, B6},
    Specifier,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[binrw]
#[brw(repr = u16, big)]
#[derive(Default, Debug, FromPrimitive, ToPrimitive, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u16)]
pub enum VideoSource {
    Black = 0,

    Input1 = 1,
    Input2 = 2,
    Input3 = 3,
    Input4 = 4,
    Input5 = 5,
    Input6 = 6,
    Input7 = 7,
    Input8 = 8,
    Input9 = 9,
    Input10 = 10,
    Input11 = 11,
    Input12 = 12,
    Input13 = 13,
    Input14 = 14,
    Input15 = 15,
    Input16 = 16,
    Input17 = 17,
    Input18 = 18,
    Input19 = 19,
    Input20 = 20,
    Input21 = 21,
    Input22 = 22,
    Input23 = 23,
    Input24 = 24,
    Input25 = 25,
    Input26 = 26,
    Input27 = 27,
    Input28 = 28,
    Input29 = 29,
    Input30 = 30,
    Input31 = 31,
    Input32 = 32,
    Input33 = 33,
    Input34 = 34,
    Input35 = 35,
    Input36 = 36,
    Input37 = 37,
    Input38 = 38,
    Input39 = 39,
    Input40 = 40,

    ColourBars = 1000,
    Colour1 = 2001,
    Colour2 = 2002,
    // TODO: Colour3 - Colour8 haven't actually been seen, these are just a
    // guess.
    Colour3 = 2003,
    Colour4 = 2004,
    Colour5 = 2005,
    Colour6 = 2006,
    Colour7 = 2007,
    Colour8 = 2008,

    MediaPlayer1 = 3010,
    MediaPlayer1Key = 3011,
    MediaPlayer2 = 3020,
    MediaPlayer2Key = 3021,
    MediaPlayer3 = 3030,
    MediaPlayer3Key = 3031,
    MediaPlayer4 = 3040,
    MediaPlayer4Key = 3041,

    Key1Mask = 4010,
    Key2Mask = 4020,
    Key3Mask = 4030,
    Key4Mask = 4040,

    DSK1Mask = 5010,
    DSK2Mask = 5020,

    SuperSource = 6000,

    CleanFeed1 = 7001,
    CleanFeed2 = 7002,

    Auxilary1 = 8001,
    Auxilary2 = 8002,
    Auxilary3 = 8003,
    Auxilary4 = 8004,
    Auxilary5 = 8005,
    Auxilary6 = 8006,

    ME1Prog = 10010,
    ME1Prev = 10011,
    ME2Prog = 10020,
    ME2Prev = 10021,

    Input1Direct = 11001,
    /// Internal value: unknown video source state.
    #[default]
    Unknown = 0xffff,
}

/// The external port type of the video switcher.
///
/// ## Format
///
/// The value is a bitmask, and uses the network port type numbering, which
/// is different from `BMDSwitcherExternalPortType` in the official SDK:
///
/// API value | Network value | Notes
/// --------- | ------------- | -----------
/// `0x0e1f`  | `0x0e1f`      | Same in both
/// `0x01c0`  | `0x00e0`      | `API = Network << 1`
/// `0x0020`  | `0x0100`      | `API = Network >> 3`
/// `0xf000`  | `0xf000`      | unused[^1]
///
/// [^1]: `0x1000` (RJ45) is unused by the BM SDK.
#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Specifier, BinRead, BinWrite, Debug, Default, PartialEq, Eq, Clone, Copy)]
#[br(map = From::<u16>::from)]
#[bw(map = |&x| Into::<u16>::into(x))]
pub struct ExternalPortType {
    /// Serial digital interface (SDI)
    pub sdi: bool,

    /// High-definition multimedia interface (HDMI)
    pub hdmi: bool,

    /// Component video
    pub component: bool,

    /// Composite video
    pub composite: bool,

    /// S-Video
    pub svideo: bool,

    /// XLR audio connection
    pub xlr: bool,

    /// AES EBU audio connection
    pub aes_ebu: bool,

    /// RCA audio connection
    pub rca: bool,

    /// Internal port
    pub internal: bool,

    /// TS audio connection
    pub ts_jack: bool,

    /// MADI audio connection
    pub madi: bool,

    /// TRS audio connection
    pub trs: bool,

    // RJ45 not supported on network layer?
    #[skip]
    __: B4,
}

/// Switcher port types
#[binrw]
#[brw(big, repr = u8)]
#[derive(Debug, FromPrimitive, ToPrimitive, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum PortType {
    /// External port (see [ExternalPortType] for specifics).
    External = 0x00,
    /// Black video generator port.
    Black = 0x01,
    /// Colour bars generator port.
    ColourBars = 0x02,
    /// Colour generator port.
    ColourGenerator = 0x03,
    /// Media player fill port.
    MediaPlayerFill = 0x04,
    /// Media player cut port.
    MediaPlayerKey = 0x05,
    /// SuperSource port.
    SuperSource = 0x06,
    /// External direct-mode port, which bypasses all switching.
    ExternalDirect = 0x07,
    /// Mix effect block output port.
    MEOutput = 0x80,
    /// Auxiliary output port.
    Auxiliary = 0x81,
    // TODO: may be "key cut output"?
    Mask = 0x82,
    /// MultiView output port.
    Multiview = 0x83,
}

/// Source tally status.
#[bitfield(bits = 8)]
#[repr(u8)]
#[derive(Specifier, BinRead, BinWrite, Debug, Default, PartialEq, Eq, Clone, Copy)]
#[br(map = From::<u8>::from)]
#[bw(map = |&x| Into::<u8>::into(x))]
pub struct TallyFlags {
    /// The source is currently in use as a program output.
    pub program: bool,
    /// The source is currently in use as a preview output.
    pub preview: bool,
    #[skip]
    __: B6,
}

/// Input/output video mode
///
/// ### Protocol notes
///
/// This enum uses the wire format.
///
/// SDK headers define modes in FourCCs as `_BMDSwitcherVideoMode`.
///
/// BMDSwitcherAPI converts it to wire formats in `CBMDSwitcher::ToBepVideoMode()`. This has a
/// big `switch` statement.
///
/// The reverse process appears in `CBMDSwitcher::ToBmdVideoMode()`, which points a `0x22` entry
/// array of:
///
/// * `uint32_t`: SDK FourCC
/// * `uint32_t`: Wire format
#[binrw]
#[brw(repr = u8)]
#[derive(Debug, FromPrimitive, ToPrimitive, PartialEq, Eq, Clone, Copy, Default)]
#[repr(u8)]
pub enum VideoMode {
    /// 525-line NTSC at 4:3 AR
    Ntsc525i59_94 = 0x0, // [0x0]
    /// 625-line PAL at 4:3 AR
    Pal625i50 = 0x1, // [0x1]
    /// 525-line NTSC at 16:9 AR
    NtscAnamorphic525i59_94 = 0x2, // [0x2]
    /// 625-line PAL at 16:9 AR
    PalAnamorphic625i50 = 0x3, // [0x3]

    /// 720p50 HD
    Hd720p50 = 0x4, // [0x4]
    /// 720p59.94 HD
    Hd720p59_94 = 0x5, // [0x5]
    /// 720p60 HD
    Hd720p60 = 0x1c, // [0x6]

    /// 1080i50 Full HD
    Fhd1080i50 = 0x6, // [0x7]
    /// 1080i59.94 Full HD
    Fhd1080i59_94 = 0x7, // [0x8]
    /// 1080i60 Full HD
    Fhd1080i60 = 0x1d, // [0x9]

    /// 1080p23.98 Full HD
    Fhd1080p23_98 = 0x8, // [0xa]
    /// 1080p24 Full HD
    Fhd1080p24 = 0x9, // [0xb]
    /// 1080p25 Full HD
    Fhd1080p25 = 0xa, // [0xc]
    /// 1080p29.97 Full HD
    Fhd1080p29_97 = 0xb, // [0xd]
    /// 1080p30 Full HD
    Fhd1080p30 = 0x1a, // [0xe]
    /// 1080p50 Full HD
    Fhd1080p50 = 0xc, // [0xf]
    /// 1080p59.94 Full HD
    Fhd1080p59_94 = 0xd, // [0x10]
    /// 1080p60 Full HD
    Fhd1080p60 = 0x1b, // [0x11]

    /// 2160p23.98 (4K) Ultra HD
    Uhd4Kp23_98 = 0xe, // [0x12]
    /// 2160p24 (4K) Ultra HD
    Uhd4Kp24 = 0xf, // [0x13]
    /// 2160p25 (4K) Ultra HD
    Uhd4Kp25 = 0x10, // [0x14]
    /// 2160p29.97 (4K) Ultra HD
    Uhd4Kp29_97 = 0x11, // [0x15]
    /// 2160p30 (4K) Ultra HD
    Uhd4Kp30 = 0x1e, // [0x16]
    /// 2160p50 (4K) Ultra HD
    Uhd4Kp50 = 0x12, // [0x17]
    /// 2160p59.94 (4K) Ultra HD
    Uhd4Kp59_94 = 0x13, // [0x18]
    /// 2160p60 (4K) Ultra HD
    Uhd4Kp60 = 0x1f, // [0x19]

    /// 4320p23.98 (8K) Ultra HD
    Uhd8Kp23_98 = 0x14, // [0x1a]
    /// 4320p24 (8K) Ultra HD
    Uhd8Kp24 = 0x15, // [0x1b]
    /// 4320p25 (8K) Ultra HD
    Uhd8Kp25 = 0x16, // [0x1c]
    /// 4320p29.97 (8K) Ultra HD
    Uhd8Kp29_97 = 0x17, // [0x1d]
    /// 4320p30 (8K) Ultra HD
    Uhd8Kp30 = 0x20, // [0x1e]
    /// 4320p50 (8K) Ultra HD
    Uhd8Kp50 = 0x18, // [0x1f]
    /// 4320p59.94 (8K) Ultra HD
    Uhd8Kp59_94 = 0x19, // [0x20]
    /// 4320p60 (8K) Ultra HD
    Uhd8Kp60 = 0x21, // [0x21]

    /// Unknown / invalid video mode value.
    #[default]
    Unknown = 0xff,
}

impl VideoMode {
    /// Total number of lines in the image.
    pub const fn lines(&self) -> u32 {
        match self {
            Self::Ntsc525i59_94 | Self::NtscAnamorphic525i59_94 => 525,
            Self::Pal625i50 | Self::PalAnamorphic625i50 => 625,
            Self::Hd720p50 | Self::Hd720p59_94 | Self::Hd720p60 => 720,
            Self::Fhd1080i50
            | Self::Fhd1080i59_94
            | Self::Fhd1080i60
            | Self::Fhd1080p23_98
            | Self::Fhd1080p24
            | Self::Fhd1080p25
            | Self::Fhd1080p29_97
            | Self::Fhd1080p30
            | Self::Fhd1080p50
            | Self::Fhd1080p59_94
            | Self::Fhd1080p60 => 1080,
            Self::Uhd4Kp23_98
            | Self::Uhd4Kp24
            | Self::Uhd4Kp25
            | Self::Uhd4Kp29_97
            | Self::Uhd4Kp30
            | Self::Uhd4Kp50
            | Self::Uhd4Kp59_94
            | Self::Uhd4Kp60 => 2160,
            Self::Uhd8Kp23_98
            | Self::Uhd8Kp24
            | Self::Uhd8Kp25
            | Self::Uhd8Kp29_97
            | Self::Uhd8Kp30
            | Self::Uhd8Kp50
            | Self::Uhd8Kp59_94
            | Self::Uhd8Kp60 => 4320,
            Self::Unknown => 0,
        }
    }

    /// Width of the image in pixels
    pub const fn width(&self) -> u32 {
        match self {
            Self::Ntsc525i59_94
            | Self::NtscAnamorphic525i59_94
            | Self::Pal625i50
            | Self::PalAnamorphic625i50 => 720,
            Self::Hd720p50 | Self::Hd720p59_94 | Self::Hd720p60 => 1280,
            Self::Fhd1080i50
            | Self::Fhd1080i59_94
            | Self::Fhd1080i60
            | Self::Fhd1080p23_98
            | Self::Fhd1080p24
            | Self::Fhd1080p25
            | Self::Fhd1080p29_97
            | Self::Fhd1080p30
            | Self::Fhd1080p50
            | Self::Fhd1080p59_94
            | Self::Fhd1080p60 => 1920,
            Self::Uhd4Kp23_98
            | Self::Uhd4Kp24
            | Self::Uhd4Kp25
            | Self::Uhd4Kp29_97
            | Self::Uhd4Kp30
            | Self::Uhd4Kp50
            | Self::Uhd4Kp59_94
            | Self::Uhd4Kp60 => 3840,
            Self::Uhd8Kp23_98
            | Self::Uhd8Kp24
            | Self::Uhd8Kp25
            | Self::Uhd8Kp29_97
            | Self::Uhd8Kp30
            | Self::Uhd8Kp50
            | Self::Uhd8Kp59_94
            | Self::Uhd8Kp60 => 7680,
            Self::Unknown => 0,
        }
    }

    /// Number of pixels in a complete image
    pub const fn pixels(&self) -> u32 {
        self.lines() * self.width()
    }

    /// Returns `true` if the mode is interlaced.
    pub const fn is_interlaced(&self) -> bool {
        matches!(
            self,
            Self::Ntsc525i59_94
                | Self::NtscAnamorphic525i59_94
                | Self::Pal625i50
                | Self::PalAnamorphic625i50
                | Self::Fhd1080i50
                | Self::Fhd1080i59_94
                | Self::Fhd1080i60
        )
    }

    /// Returns the mode's number of fields or frames per 100 seconds.
    ///
    /// ie: `5000` = 50 frames or fields per second
    pub const fn rate_per_100sec(&self) -> u16 {
        match self {
            Self::Ntsc525i59_94
            | Self::Hd720p59_94
            | Self::Fhd1080i59_94
            | Self::Fhd1080p59_94
            | Self::Uhd4Kp59_94
            | Self::Uhd8Kp59_94
            | Self::NtscAnamorphic525i59_94 => 59_94,

            Self::Pal625i50
            | Self::Hd720p50
            | Self::Fhd1080i50
            | Self::Fhd1080p50
            | Self::Uhd4Kp50
            | Self::Uhd8Kp50
            | Self::PalAnamorphic625i50 => 50_00,

            Self::Hd720p60
            | Self::Fhd1080i60
            | Self::Fhd1080p60
            | Self::Uhd4Kp60
            | Self::Uhd8Kp60 => 60_00,

            Self::Fhd1080p23_98 | Self::Uhd4Kp23_98 | Self::Uhd8Kp23_98 => 23_98,
            Self::Fhd1080p24 | Self::Uhd4Kp24 | Self::Uhd8Kp24 => 24_00,
            Self::Fhd1080p25 | Self::Uhd4Kp25 | Self::Uhd8Kp25 => 25_00,
            Self::Fhd1080p29_97 | Self::Uhd4Kp29_97 | Self::Uhd8Kp29_97 => 29_97,
            Self::Fhd1080p30 | Self::Uhd4Kp30 | Self::Uhd8Kp30 => 30_00,
            Self::Unknown => 0,
        }
    }

    /// Returns `true` if the video mode is [anamorphic format][0]
    /// (stretched to 16:9).
    ///
    /// [0]: https://en.wikipedia.org/wiki/Anamorphic_format
    pub const fn is_anamorphic(&self) -> bool {
        matches!(
            self,
            Self::PalAnamorphic625i50 | Self::NtscAnamorphic525i59_94
        )
    }
}

impl Display for VideoMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if matches!(self, Self::Unknown) {
            return f.write_str("(Unknown)");
        }

        let (i, r) = (self.rate_per_100sec() / 100, self.rate_per_100sec() % 100);
        let frac = if r == 0 {
            String::new()
        } else {
            format!(".{:02}", r)
        };
        f.write_fmt(format_args!(
            "{}{}{i}{frac}{}",
            self.lines(),
            if self.is_interlaced() { "i" } else { "p" },
            if self.is_anamorphic() {
                " (Anamorphic)"
            } else {
                ""
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn video_mode_formatting() {
        assert_eq!(
            VideoMode::PalAnamorphic625i50.to_string(),
            "625i50 (Anamorphic)"
        );
        assert_eq!(VideoMode::Fhd1080i50.to_string(), "1080i50");
        assert_eq!(VideoMode::Fhd1080p23_98.to_string(), "1080p23.98");
    }

    #[test]
    fn external_port_types() {
        assert_eq!(
            ExternalPortType::new().with_sdi(true),
            ExternalPortType::from(0x0001u16)
        );
    }
}
