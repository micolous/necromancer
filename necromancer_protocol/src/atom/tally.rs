//! Tally commands
//!
//! ## Unimplemented atoms
//!
//! FourCC | Atom name | Length
//! ------ | --------- | ------
//! `_TlC` | `CapabilitiesTally` | 0x10
//! `TlIn` | `TalliedInputs` | 0xa + entries bytes
//! `DTOE` | `DskTallyOverride` | 0xc

use super::video::{TallyFlags, VideoSource};
use binrw::binrw;
use std::{collections::HashMap, ops::Deref};

/// `TlSr`: tally status by video source
///
/// This `struct` can be converted to/from a
/// [`HashMap<VideoSource, TallyFlags>`] using the [`From`] trait.
///
/// ## Packet format
///
/// * `u16`: number of tally statuses
/// * (repeated)
///   * `u16`: [video source][VideoSource]
///   * `u8`: [tally flags][TallyFlags]
///
/// *Unlike* most other structures, [`TalliedSources`] is `repr(packed)`, and
/// contains no padding.
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TalliedSources {
    #[br(temp)]
    #[bw(try_calc(u16::try_from(v.len())))]
    length: u16,

    #[br(count=length)]
    v: Vec<TallyBySourceEntry>,
}

#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct TallyBySourceEntry(VideoSource, TallyFlags);

impl TallyBySourceEntry {
    #[inline]
    pub const fn video_source(&self) -> VideoSource {
        self.0
    }

    #[inline]
    pub const fn tally_flags(&self) -> TallyFlags {
        self.1
    }
}

impl TalliedSources {
    // const HEADERS_LENGTH: usize = 2;
    // /// Maximum number of sources that may be defined in a [TallyBySource] message.
    // const MAX_SOURCES: usize = (Atom::MAX_PAYLOAD_LENGTH - Self::HEADERS_LENGTH) / 3;

    /// Get the [tally state][TallyFlags] of a given source, or [None] if not
    /// defined.
    pub fn get(&self, source: VideoSource) -> Option<TallyFlags> {
        self.iter()
            .find(|e| e.video_source() == source)
            .map(|e| e.tally_flags())
    }
}

impl From<(VideoSource, TallyFlags)> for TallyBySourceEntry {
    fn from((source, flags): (VideoSource, TallyFlags)) -> Self {
        Self(source, flags)
    }
}

impl From<TallyBySourceEntry> for (VideoSource, TallyFlags) {
    fn from(value: TallyBySourceEntry) -> Self {
        (value.0, value.1)
    }
}

impl From<Vec<TallyBySourceEntry>> for TalliedSources {
    fn from(v: Vec<TallyBySourceEntry>) -> Self {
        Self { v }
    }
}

impl From<Vec<(VideoSource, TallyFlags)>> for TalliedSources {
    fn from(value: Vec<(VideoSource, TallyFlags)>) -> Self {
        Self {
            v: value.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<TalliedSources> for Vec<TallyBySourceEntry> {
    fn from(value: TalliedSources) -> Self {
        value.v
    }
}

impl From<TalliedSources> for Vec<(VideoSource, TallyFlags)> {
    fn from(value: TalliedSources) -> Self {
        value.v.into_iter().map(Into::into).collect()
    }
}

impl Deref for TalliedSources {
    type Target = Vec<TallyBySourceEntry>;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

/// Converts a [TalliedSources] message into a [HashMap].
///
/// [HashMap]s do not preserve ordering.
impl From<TalliedSources> for HashMap<VideoSource, TallyFlags> {
    fn from(value: TalliedSources) -> Self {
        HashMap::from_iter(value.v.into_iter().map(Into::into))
    }
}

/// Converts a [HashMap] into a [TalliedSources] message.
impl From<HashMap<VideoSource, TallyFlags>> for TalliedSources {
    fn from(value: HashMap<VideoSource, TallyFlags>) -> Self {
        TalliedSources {
            v: Vec::from_iter(value.into_iter().map(Into::into)),
        }
    }
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
    fn tally_by_source() -> Result<()> {
        let expected = TalliedSources {
            v: vec![
                TallyBySourceEntry(VideoSource::Black, TallyFlags::new()),
                TallyBySourceEntry(VideoSource::Input1, TallyFlags::new().with_preview(true)),
                TallyBySourceEntry(VideoSource::Input2, TallyFlags::new()),
                TallyBySourceEntry(VideoSource::Input3, TallyFlags::new()),
                TallyBySourceEntry(VideoSource::Input4, TallyFlags::new().with_program(true)),
                TallyBySourceEntry(VideoSource::ColourBars, TallyFlags::new()),
                TallyBySourceEntry(VideoSource::Colour1, TallyFlags::new()),
                TallyBySourceEntry(VideoSource::Colour2, TallyFlags::new()),
                TallyBySourceEntry(VideoSource::MediaPlayer1, TallyFlags::new()),
                TallyBySourceEntry(VideoSource::MediaPlayer1Key, TallyFlags::new()),
                TallyBySourceEntry(VideoSource::ME1Prog, TallyFlags::new()),
                TallyBySourceEntry(VideoSource::ME1Prev, TallyFlags::new()),
                TallyBySourceEntry(VideoSource::Input1Direct, TallyFlags::new()),
                TallyBySourceEntry(VideoSource::Auxilary1, TallyFlags::new()),
            ],
        };
        let cmd = hex::decode("00340000546c5372000e00000000010200020000030000040103e80007d10007d2000bc2000bc300271a00271b002af9001f4100")?;
        let tlsr = Atom::read(&mut Cursor::new(&cmd))?;

        let Payload::TalliedSources(tlsr) = tlsr.payload else {
            panic!("wrong command type");
        };

        assert_eq!(expected, tlsr);
        assert_eq!(
            Some(TallyFlags::new().with_preview(true)),
            tlsr.get(VideoSource::Input1)
        );
        assert_eq!(Some(TallyFlags::new()), tlsr.get(VideoSource::Input2));
        assert_eq!(None, tlsr.get(VideoSource::Auxilary2));

        let o = Atom::new(expected.into());
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }
}
