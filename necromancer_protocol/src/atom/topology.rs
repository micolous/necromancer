//! # Topology
use binrw::binrw;

/// `_top`: Topology of the switcher (`CapabilitiesTopLevel`)
#[binrw]
#[derive(Default, Debug, PartialEq, Eq, Clone)]
#[brw(big)]
pub struct Topology {
    /// Number of mix effect blocks (MEs)
    pub mes: u8,
    /// Number of input sources
    pub sources: u8,
    pub downstream_keys: u8,
    pub auxs: u8,
    pub mix_minus_outputs: u8,
    /// Number of media players
    pub media_players: u8,
    /// Number of multiview outputs
    pub multiviewers: u8,
    /// Number of serial ports
    pub serial_ports: u8,
    /// Number of HyperDecks
    pub hyperdecks: u8,
    // Skaarhoj says "Has SD Output"; can't see where this is read in the SDK,
    // and the data length is different.
    unknown9: u8,
    unknown10: u8,
    unknown11: u8,
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub audio_mixer: bool,
    unknown13: u8,
    unknown14: u8,
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub farlight_audio_mixer: bool,
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub down_conversion_methods: bool,
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub down_converted_hd_video_modes: bool,
    /// Supports camera control.
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub camera_control: bool,
    /// Supports PTZ camera control using Visca over RS-422
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub supports_serial_ptz_visca: bool,
    /// Supports Grass Valley (GVG100) control over RS-422
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub supports_serial_gvg100: bool,
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub sdi3g: bool,
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub supports_advanced_chroma: bool,
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub configurable_outputs: bool,
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub auto_video_mode: bool,
    unknown25: u8,
    unknown26: u8,
    unknown27: u8,
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use binrw::{BinRead, BinWrite};

    use crate::{
        atom::{Atom, Payload},
        Result,
    };

    use super::*;
    #[test]
    fn atem_mini() -> Result<()> {
        let expected = Topology {
            mes: 1,
            sources: 14,
            downstream_keys: 1,
            auxs: 1,
            mix_minus_outputs: 0,
            media_players: 1,
            multiviewers: 0,
            serial_ports: 0,
            hyperdecks: 4,
            audio_mixer: false,
            farlight_audio_mixer: true,
            camera_control: true,

            unknown9: 1,
            supports_advanced_chroma: true,
            configurable_outputs: true,
            auto_video_mode: true,
            ..Default::default()
        };
        let cmd = hex::decode(
            "002400005f746f70010e0101000100000401000000000001000001000000010101000000",
        )?;

        let top = Atom::read(&mut Cursor::new(&cmd))?;
        //let top = i.next()?.unwrap();
        //assert!(i.next()?.is_none());

        //assert_eq!(*b"_top", top.id);
        //assert_eq!(36, top.length);

        //let payload = top.parse_payload()?;
        let payload = top.payload;
        let Payload::Topology(top) = payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, top);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }
}
