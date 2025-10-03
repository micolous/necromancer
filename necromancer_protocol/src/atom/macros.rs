//! # Macros; 1/9 atoms
//!
//! ## Unimplemented atoms (8)
//!
//! FourCC | Atom name | Length
//! ------ | --------- | ------
//! `MAct` | `MacroAction` | 0xc
//! `MPrp` | `MacroProperties` | variable
//! `MRcS` | `MacroRecordStatus` | 0xc
//! `MRPr` | `MacroRunProperties` | 0xc
//! `MSlp` | `MacroSleep` | 0xc
//! `MSRc` | `MacroStartRecord` | variable
//! `MRCP` | `ChangeMacroRunProperties` | 0xc
//! `CMPr` | `ChangeMacroProperties` | variable

use binrw::binrw;

/// `_MAC`: Macro capabilities (`CapabilitiesMacros`)
///
/// ## Packet format
///
/// * `u8`: number of macros which the switcher can store
/// * 3 bytes padding
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MacroCapabilities {
    #[brw(pad_size_to = 4)]
    pub count: u8,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{atom::Atom, Result};
    use binrw::BinRead;
    use std::io::Cursor;

    #[test]
    fn mac() -> Result {
        // ATEM Mini
        let cmd = hex::decode("000c00005f4d414364000000")?;
        let cmd = Atom::read(&mut Cursor::new(&cmd))?;

        let expected = Atom::new(MacroCapabilities { count: 100 });
        assert_eq!(expected, cmd);

        Ok(())
    }
}
