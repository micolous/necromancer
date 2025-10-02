//! # Start-up settings; 3/3 atoms
use binrw::binrw;

/// Command to save the current settings to the start-up configuration.
pub const SAVE_STARTUP_SETTINGS: SaveSettings = SaveSettings { slot: 0 };

/// Command to restore the start-up configuration.
///
/// **Warning:** this is never sent by the SDK.
pub const RESTORE_STARTUP_SETTINGS: RestoreSettings = RestoreSettings { slot: 0 };

/// Command to clear the start-up configuration.
pub const CLEAR_STARTUP_SETTINGS: ClearSettings = ClearSettings { slot: 0 };

/// `SRsv`: Save settings (`SaveRecallSaveSettings`)
///
/// ## Packet format
///
/// * `u8`: Slot? Always 0 (start-up config)
/// * 3 bytes padding
#[binrw]
#[brw(big)]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct SaveSettings {
    #[brw(pad_after = 3)]
    pub slot: u8,
}

/// `SRrs`: Restore settings (`SaveRecallRestoreSettings`)
///
/// ## Packet format
///
/// Command doesn't seem to be sent by the SDK anywhere, so assuming this is the
/// same as [SaveSettings] and [ClearSettings]
///
/// * `u8`: Slot? Always 0 (start-up config)
/// * 3 bytes padding
#[binrw]
#[brw(big)]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct RestoreSettings {
    #[brw(pad_after = 3)]
    pub slot: u8,
}

/// `SRcl`: Clear settings (`SaveRecallClearSettings`)
///
/// ## Packet format
///
/// * `u8`: Slot? Always 0 (start-up config)
/// * 3 bytes padding
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
#[binrw]
#[brw(big)]
pub struct ClearSettings {
    #[brw(pad_after = 3)]
    pub slot: u8,
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
    fn save_settings() -> Result<()> {
        let expected = SaveSettings { slot: 0 };
        let cmd = hex::decode("000c00005352737600000000")?;
        let srsv = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::SaveSettings(srsv) = srsv.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, srsv);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn clear_settings() -> Result<()> {
        let expected = ClearSettings { slot: 0 };
        let cmd = hex::decode("000c00005352636c00000000")?;
        let srcl = Atom::read(&mut Cursor::new(&cmd))?;
        let Payload::ClearSettings(srcl) = srcl.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, srcl);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }
}
