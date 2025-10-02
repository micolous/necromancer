//! # Manufacturing tests; 2/2 atoms
//!
//! **Warning:** public ATEM software doesn't call these APIs, so functionality
//! is unknown. This could damage your device.
use binrw::binrw;

/// **Warning:** public ATEM software doesn't call these APIs, so functionality
/// is unknown. This could damage your device.
#[binrw]
#[brw(repr = u8)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum TestOperation {
    #[default]
    None = 0x0,
    Rs42 = 0x1,
    Fmem = 0x2,
    Hdmi = 0x3,
    Rsta = 0x4,
    Btns = 0x5,
    // no result type?
    Lcda = 0x6,
    Tcmt = 0x7,
    Leds = 0x8,
    Lcds = 0x9,
    Dial = 0xa,
    Ball = 0xb,
    Tbar = 0xc,
    // no result type?
    Hpic = 0xd,
    UsCn = 0xe,
    Hcec = 0xf,
}

/// `MfgT`: manufacturing test (`MfgDoTest`)
///
/// **Warning:** public ATEM software doesn't call these APIs, so functionality
/// is unknown. This could damage your device.
///
/// ## Packet format
///
/// * `u8`: test operation
/// * 3 bytes padding
/// * `u32`: unknown parameter
#[binrw]
#[brw(big)]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct MfgTest {
    #[brw(pad_after = 3)]
    operation: TestOperation,
    param: u32,
}

/// `MfgR`: manufacturing test result (`MfgTestResult`)
///
/// **Warning:** public ATEM software doesn't call these APIs, so functionality
/// is unknown. This could damage your device.
///
/// ## Packet format
///
/// * `u8`: test type
/// * `u8`: result code?
/// * 2 bytes padding
#[binrw]
#[brw(big)]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct MfgTestResult {
    operation: TestOperation,
    #[brw(pad_after = 2)]
    result: u8,
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
    fn mfg_test() -> Result<()> {
        // This test is pure speculation
        let expected = MfgTest {
            operation: TestOperation::Ball,
            param: 1,
        };
        let cmd: Vec<u8> = hex::decode("001000004d6667540b00000000000001")?;
        let mfgt = Atom::read(&mut Cursor::new(&cmd))?;

        let Payload::MfgTest(mfgt) = mfgt.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, mfgt);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn mfg_test_result() -> Result<()> {
        // This test is pure speculation
        let expected = MfgTestResult {
            operation: TestOperation::Ball,
            result: 1,
        };
        let cmd: Vec<u8> = hex::decode("000c00004d6667520b010000")?;
        let mfgr = Atom::read(&mut Cursor::new(&cmd))?;

        let Payload::MfgTestResult(mfgr) = mfgr.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, mfgr);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }
}
