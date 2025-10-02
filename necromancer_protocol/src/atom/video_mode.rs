//! # Video mode; 3/6 atoms
//!
//! ## Unimplemented atoms
//!
//! FourCC | Atom name | Length
//! ------ | --------- | ------
//! `_VML` | `VideoModeDescriptorList` | 0xc + (entries * 0xc)
//! `AiVM` | `AutoVideoMode` | 0xc
//! `VMC2` | `CapabilitiesVideoModeExtended` | 0xc + (entries * 0x10)
use crate::structs::VideoMode;
use binrw::binrw;
use std::ops::Deref;

/// `VidM`: current video mode (`CoreVideoMode`)
///
/// ## Packet format
///
/// * `u8`: Current video mode
/// * 3 bytes padding
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoreVideoMode(#[brw(pad_size_to = 4)] pub VideoMode);

impl From<VideoMode> for CoreVideoMode {
    fn from(value: VideoMode) -> Self {
        CoreVideoMode(value)
    }
}

impl From<CoreVideoMode> for VideoMode {
    fn from(value: CoreVideoMode) -> Self {
        value.0
    }
}

impl Deref for CoreVideoMode {
    type Target = VideoMode;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// `CVdM`: set video mode (`ChangeCoreVideoMode`)
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetVideoMode(#[brw(pad_size_to = 4)] pub VideoMode);

impl From<VideoMode> for SetVideoMode {
    fn from(value: VideoMode) -> Self {
        SetVideoMode(value)
    }
}

impl From<SetVideoMode> for VideoMode {
    fn from(value: SetVideoMode) -> Self {
        value.0
    }
}

impl Deref for SetVideoMode {
    type Target = VideoMode;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// `_VMC`: supported video modes (`CapabilitiesVideoMode`)
///
/// ## Packet format
///
/// * `u16`: number of supported modes
/// * 2 bytes padding
/// * (repeated)
///   * `u8`: video mode
///   * 12 bytes padding
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SupportedVideoModes {
    #[brw(pad_after = 2)]
    #[br(temp)]
    #[bw(try_calc(u16::try_from(modes.len())))]
    length: u16,

    #[br(count = length)]
    pub modes: Vec<SupportedVideoMode>,
}

#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SupportedVideoMode(#[brw(pad_size_to = 13)] pub VideoMode);

impl From<VideoMode> for SupportedVideoMode {
    fn from(value: VideoMode) -> Self {
        SupportedVideoMode(value)
    }
}

impl From<SupportedVideoMode> for VideoMode {
    fn from(value: SupportedVideoMode) -> Self {
        value.0
    }
}

impl Deref for SupportedVideoMode {
    type Target = VideoMode;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<VideoMode>> for SupportedVideoModes {
    fn from(value: Vec<VideoMode>) -> Self {
        value
            .into_iter()
            .map(Into::into)
            .collect::<Vec<SupportedVideoMode>>()
            .into()
    }
}

impl From<Vec<SupportedVideoMode>> for SupportedVideoModes {
    fn from(modes: Vec<SupportedVideoMode>) -> Self {
        Self { modes }
    }
}

impl From<SupportedVideoModes> for Vec<VideoMode> {
    fn from(value: SupportedVideoModes) -> Self {
        value.modes.into_iter().map(Into::into).collect()
    }
}

impl From<SupportedVideoModes> for Vec<SupportedVideoMode> {
    fn from(value: SupportedVideoModes) -> Self {
        value.modes
    }
}

impl Deref for SupportedVideoModes {
    type Target = Vec<SupportedVideoMode>;

    fn deref(&self) -> &Self::Target {
        &self.modes
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        atom::{Atom, Payload},
        Result,
    };
    use binrw::{BinRead, BinWrite};
    use std::io::Cursor;

    #[test]
    fn supported_video_modes() -> Result<()> {
        let expected = SupportedVideoModes::from(vec![
            VideoMode::Fhd1080p23_98,
            VideoMode::Fhd1080p24,
            VideoMode::Fhd1080p25,
            VideoMode::Fhd1080p29_97,
            VideoMode::Fhd1080p30,
            VideoMode::Fhd1080p50,
            VideoMode::Fhd1080p59_94,
            VideoMode::Fhd1080p60,
        ]);
        let cmd = hex::decode(concat!(
            "00740000",
            "5f564d43",
            "00080000",
            "08000000000000000000000000",
            "09000000000000000000000000",
            "0a000000000000000000000000",
            "0b000000000000000000000000",
            "1a000000000000000000000000",
            "0c000000000000000000000000",
            "0d000000000000000000000000",
            "1b000000000000000000000000",
        ))?;
        let vmc = Atom::read(&mut Cursor::new(&cmd))?;

        let Payload::SupportedVideoModes(vmc) = vmc.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, vmc);
        assert!(vmc.contains(&VideoMode::Fhd1080p60.into()));

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }
}
