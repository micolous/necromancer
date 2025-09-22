//! Cut and auto (transition) atoms
//!
//! ## Unimplemented atoms
//!
//! FourCC | Atom name | Length
//! ------ | --------- | ------
//! `DAu2` | `DoTransitionAuto_2` | 0xc

use binrw::binrw;

/// `DCut`: cut (`DoTransitionCut`)
///
/// Swap program and preview inputs immediately.
///
/// ## Packet format
///
/// * `u8`: ME to cut
/// * 3 bytes padding
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cut {
    #[brw(pad_size_to = 4)]
    pub me: u8,
}

/// `DAut`: auto (`DoTransitionAuto`)
///
/// Swap program and preview inputs with a transition.
///
/// ## Packet format
///
/// * `u8`: ME to auto
/// * 3 bytes padding
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Auto {
    #[brw(pad_size_to = 4)]
    pub me: u8,
}

/// `TrPs`: transition position
///
/// ## Packet format
///
/// * `u8`: ME
/// * `bool`: transition in progress
/// * `u8`: number of frames of transition remaining
/// * 1 byte padding
/// * `u16`: transition position
/// * 2 bytes padding
#[binrw]
#[brw(big)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TransitionPosition {
    pub me: u8,
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub in_progress: bool,
    #[brw(pad_after = 1)]
    pub frames_remaining: u8,
    /// Transition position (0..=10000)
    #[brw(pad_after = 2)]
    pub position: u16,
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use binrw::{BinRead, BinWrite};

    use crate::{
        atom::{Atom, Payload},
        packet::{AtemPacket, AtemPacketFlags},
        Result,
    };

    use super::*;

    #[test]
    fn cut() -> Result<()> {
        let cmd = hex::decode("08188001000000000001000f000c00004443757400000000")?;
        let pkt = AtemPacket::read(&mut Cursor::new(&cmd))?;
        let payload = pkt.atoms().expect("wrong payload type");

        assert_eq!(1, payload.len());
        let Payload::Cut(cut) = &payload[0].payload else {
            panic!("wrong command type");
        };

        assert_eq!(0, cut.me);

        let o = AtemPacket::new_atoms(
            AtemPacketFlags::new().with_ack(true),
            0x8001,
            0,
            0x1,
            0xf,
            vec![Atom::new(Cut { me: 0 }.into())],
        );
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;

        assert_eq!(cmd, out.into_inner());
        Ok(())
    }
}
