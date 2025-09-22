use binrw::binrw;
use std::fmt::Display;

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
    use num_traits::FromPrimitive;

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
    fn interlaced() {
        for m in [0, 1, 2, 3, 6, 7, 0x1d] {
            let m = VideoMode::from_u8(m).unwrap();
            assert!(m.is_interlaced(), "mode {m:?} must be interlaced");
        }

        for m in [4, 5, 0xc, 0xd, 0x21] {
            let m = VideoMode::from_u8(m).unwrap();
            assert!(!m.is_interlaced(), "mode {m:?} must be progressive");
        }
    }
}
