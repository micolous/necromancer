use binrw::binrw;

/// `InCm`: initialisation complete
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InitialisationComplete {
    pub unknown1: u8,
    #[brw(pad_after = 2)]
    pub unknown2: u8,
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
    fn initialisation_complete() -> Result<()> {
        let expected = InitialisationComplete {
            unknown1: 1,
            unknown2: 64,
        };
        let cmd = hex::decode("000c0000496e436d01400000")?;
        let ct = Atom::read(&mut Cursor::new(&cmd))?;

        let Payload::InitialisationComplete(incm) = ct.payload else {
            panic!("wrong command type");
        };

        assert_eq!(expected, incm);

        let o = Atom::new(expected.into());
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());

        Ok(())
    }
}
