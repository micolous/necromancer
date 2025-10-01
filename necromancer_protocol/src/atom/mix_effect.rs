//! Mix effect block atoms (preview / program output control)
//!
use crate::structs::VideoSource;
use binrw::binrw;

/// `_MeC`: mix effect block capabilities (`CapabilitiesMEBlock`)
///
/// ## Packet format
///
/// * `u8`: ME
/// * `u8`: number of keyers on ME
#[binrw]
#[brw(big)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MixEffectBlockCapabilities {
    pub me: u8,
    #[brw(pad_after = 2)]
    pub keyers: u8,
}

/// `PrvI`: preview input selection
///
/// Sent by the switcher to indicate when the preview input for an ME has changed.
///
/// ## Packet format
///
/// * `u8`: ME
/// * 1 byte padding
/// * `u16`: video source
/// * `bool`: preview input is live
/// * 3 bytes padding
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreviewInput {
    #[brw(pad_after = 1)]
    pub me: u8,
    pub video_source: VideoSource,
    #[brw(pad_after = 3)]
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub preview_input_live: bool,
}

/// `CPvI`: set preview input (`ChangePreviewInput`)
///
/// Sent by a client to change the preview input.
///
/// ## Packet format
///
/// * `u8`: ME
/// * 1 byte padding
/// * `u16`: video source
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetPreviewInput {
    #[brw(pad_after = 1)]
    pub me: u8,
    pub video_source: VideoSource,
}

/// `PrgI`: program input selection
///
/// Sent by the switcher to indicate when the program input for an ME has changed.
///
/// ## Packet format
///
/// * `u8`: ME
/// * 1 byte padding
/// * `u16`: video source
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramInput {
    #[brw(pad_after = 1)]
    pub me: u8,
    pub video_source: VideoSource,
}

/// `CPgI`: set program input (`ChangeProgramInput`)
///
/// Sent by a client to change the program input.
///
/// ## Packet format
///
/// * `u8`: ME
/// * 1 byte padding
/// * `u16`: video source
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetProgramInput {
    #[brw(pad_after = 1)]
    pub me: u8,
    pub video_source: VideoSource,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::atom::{Atom, Payload};
    use crate::Result;
    use binrw::{BinRead, BinWrite};
    use std::io::Cursor;

    #[test]
    fn mix_effect_caps() -> Result<()> {
        let expected = MixEffectBlockCapabilities { me: 0, keyers: 1 };
        let cmd = hex::decode("000c00005f4d654300010000")?;
        let mec = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::MixEffectBlockCapabilities(mec) = mec.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, mec);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn prewiew_input() -> Result<()> {
        let expected = PreviewInput {
            me: 0,
            video_source: VideoSource::Input2,
            preview_input_live: false,
        };
        // uninitialized memory removed, grumble grumble
        let cmd = hex::decode("00100000507276490000000200000000")?;
        let preview = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::PreviewInput(preview) = preview.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, preview);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn set_preview_input() -> Result<()> {
        let expected = SetPreviewInput {
            me: 0,
            video_source: VideoSource::Input2,
        };
        // uninitialized memory removed, grumble grumble
        let cmd = hex::decode("000c00004350764900000002")?;
        let preview = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::SetPreviewInput(preview) = preview.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, preview);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn program_input() -> Result<()> {
        let expected = ProgramInput {
            me: 0,
            video_source: VideoSource::Input1,
        };
        let cmd = hex::decode("000c00005072674900000001")?;
        let program = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::ProgramInput(program) = program.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, program);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn set_program_input() -> Result<()> {
        let expected = SetProgramInput {
            me: 0,
            video_source: VideoSource::Input3,
        };
        let cmd = hex::decode("000c00004350674900000003")?;
        let program = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::SetProgramInput(program) = program.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, program);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }
}
