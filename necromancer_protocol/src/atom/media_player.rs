//! Media player atoms
//!
//! ## Unimplemented atoms
//!
//! FourCC | Atom name | Length
//! ------ | --------- | ------
//! `MPSp` | `MediaPlayerSetup` | 0x14
//! `SMPC` | `SetMediaPlayerClip` | 0x4c
//! `SMPA` | `SetMediaPlayerAudio` | 0x4c
//! `SMPS` | `SetMediaPlayerStill` | 0x4c

use binrw::binrw;

pub const CAPTURE_STILL: CaptureStill = CaptureStill {};

/// `Capt`: capture still image from ME
///
/// ## Packet format
///
/// No payload.
#[binrw]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct CaptureStill {}

#[binrw]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MediaPlayerSourceID {
    #[brw(magic = 1u8)]
    Still(#[brw(pad_after = 1)] u8),
    #[brw(magic = 2u8)]
    VideoClip(#[brw(pad_before = 1)] u8),
}

/// `MPCE`: Media player source change event
///
/// ## Packet format
///
/// * `u8`: media player ID
/// * `u8`: type (0x01 = still, 0x02 = clip)
/// * `u8`: still image index
/// * `u8`: video clip index
#[binrw]
#[brw(big)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MediaPlayerSource {
    /// Media player ID
    pub id: u8,
    pub source: MediaPlayerSourceID,
}

/// `MPSS`: Set media player source
///
/// ## Packet format
///
/// * `u8`: setting mask:
///   * 0x01: enable
///   * 0x02: still image
///   * 0x04: video clip
/// * `u8`: media player ID
/// * `u8`: type (0x01 = still, 0x02 = clip)
/// * `u8`: still image index
/// * `u8`: video clip index
/// * 3 bytes padding
#[binrw]
#[brw(big)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SetMediaPlayerSource {
    // We only read the "enable" bit from this, but write all bits.
    #[br(temp)]
    #[bw(calc(self.mask()))]
    setting_mask: u8,

    #[br(calc(setting_mask & 0x01 != 0))]
    #[bw(ignore)]
    pub enable: bool,

    /// Media player ID to control
    pub id: u8,
    // pub enable: bool,
    #[brw(pad_after = 3)]
    pub source: MediaPlayerSourceID,
}

impl SetMediaPlayerSource {
    /// Calculates the `setting_mask` for this [SetMediaPlayerSource] command.
    fn mask(&self) -> u8 {
        let mut o = 0;
        if self.enable {
            o |= 0x01;
        }
        match self.source {
            MediaPlayerSourceID::Still(_) => {
                o |= 0x02;
            }
            MediaPlayerSourceID::VideoClip(_) => {
                o |= 0x04;
            }
        }
        o
    }
}

/// `_mpl`: Media player capabilities
///
/// ## Packet format
///
/// * `u8`: still frame count
/// * `u8`: clip count
/// * `bool`: supports still image capture
/// * `u8`: ?
#[binrw]
#[brw(big)]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct MediaPlayerCapabilities {
    pub still_count: u8,
    pub clip_count: u8,
    #[brw(pad_after = 1)]
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub supports_still_capture: bool,
}

#[cfg(test)]
mod test {
    use binrw::{BinRead, BinWrite};
    use std::io::Cursor;

    use super::*;
    use crate::{
        atom::{Atom, Payload},
        Result,
    };

    #[test]
    fn capture() -> Result<()> {
        let cmd: Vec<u8> = hex::decode("0008000043617074")?;
        let capture = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::CaptureStill(capture) = capture.payload else {
            panic!("wrong command type");
        };
        assert_eq!(CAPTURE_STILL, capture);

        let o = Atom::new(CAPTURE_STILL.into());
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn media_player_source_changed() -> Result<()> {
        let expected = MediaPlayerSource {
            id: 0,
            source: MediaPlayerSourceID::Still(5),
        };
        let cmd: Vec<u8> = hex::decode("000c00004d50434500010500")?;
        let mpce = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::MediaPlayerSource(mpce) = mpce.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, mpce);

        let o = Atom::new(expected.into());
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn set_media_player_source() -> Result<()> {
        let expected = SetMediaPlayerSource {
            id: 0,
            enable: true,
            source: MediaPlayerSourceID::Still(5),
        };
        let cmd: Vec<u8> = hex::decode("001000004d5053530300010500000000")?;
        let mpss = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::SetMediaPlayerSource(mpss) = mpss.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, mpss);

        let o = Atom::new(expected.into());
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn media_player_capabilites() -> Result<()> {
        let expected = MediaPlayerCapabilities {
            still_count: 20,
            clip_count: 0,
            supports_still_capture: true,
        };
        let cmd: Vec<u8> = hex::decode("000c00005f6d706c14000100")?;
        let mpl = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::MediaPlayerCapabilities(mpl) = mpl.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, mpl);

        let o = Atom::new(expected.into());
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }
}
