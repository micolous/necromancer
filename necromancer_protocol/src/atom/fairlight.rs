//! # Fairlight audio; 2/51 atoms
//!
//! ## Unimplemented atoms
//!
//! FourCC | Atom name | Length
//! ------ | --------- | ------
//! `_FAC` | `CapabilitiesFairlightAudioMixer` | 0xc
//! `_FEC` | `CapabilitiesFairlightEqualiserBandRange` | 0xc + (0xc * frequency_limits_len)
//! `_FMH` | `CapabilitiesFairlightAudioMixerHeadphoneOut` | 0xc
//! `AEBP` | `FairlightAudioMixerInputSourceEqualiserBandProperties` | 0x2c
//! `AICP` | `FairlightAudioMixerInputSourceCompressorProperties` | 0x30
//! `AILP` | `FairlightAudioMixerInputSourceLimiterProperties` | 0x2c
//! `AIXP` | `FairlightAudioMixerInputSourceExpanderProperties` | 0x30
//! `AMBP` | `FairlightAudioMixerMasterOutEqualiserBandProperties` | 0x1c
//! `AMLP` | `FairlightAudioMixerMasterOutLimiterProperties` | 0x1c
//! `CEBP` | `ChangeFairlightAudioMixerInputSourceEqualiserBandProperties` | 0x28
//! `CFAI` | `ChangeFairlightAudioMixerAuxOutInputProperties` | 0x14
//! `CFAO` | `ChangeFairlightAudioMixerAuxOutMixerProperties` | 0x14
//! `CFEP` | `ChangeFairlightAudioMixerAnalogInputExtendedProperties` | 0x10
//! `CFIP` | `ChangeFairlightAudioMixerInputProperties` | 0x10
//! `CFMH` | `ChangeFairlightAudioMixerHeadphoneOutProperties` | 0x2c
//! `CFMP` | `ChangeFairlightAudioMixerMasterOutProperties` | 0x1c
//! `CFMS` | `ChangeFairlightAudioMixerSolo` | 0x20
//! `CFSP` | `ChangeFairlightAudioMixerInputSourceProperties` | 0x38
//! `CICP` | `ChangeFairlightAudioMixerInputSourceCompressorProperties` | 0x30
//! `CILP` | `ChangeFairlightAudioMixerInputSourceLimiterProperties` | 0x2c
//! `CILP` | `ChangeFairlightAudioMixerInputSourceLimiterProperties` | 0x2c
//! `CIXP` | `ChangeFairlightAudioMixerInputSourceExpanderProperties` | 0x30
//! `CMBP` | `ChangeFairlightAudioMixerMasterOutEqualiserBandProperties` | 0x1c
//! `CMCP` | `ChangeFairlightAudioMixerMasterOutCompressorProperties` | 0x20
//! `CMLP` | `ChangeFairlightAudioMixerMasterOutLimiterProperties` | 0x1c
//! `CMPP` | `ChangeFairlightAudioMixerProperties` | 0xc
//! `FAIC` | `CapabilitiesFairlightAudioMixerAuxOutInput` | 0x10
//! `FAIP` | `FairlightAudioMixerInputProperties` | 0x18
//! `FAMC` | `CapabilitiesFairlightAudioMixerAuxOutMixer` | 0xc
//! `FAMP` | `FairlightAudioMixerMasterOutProperties` | 0x1c
//! `FAMS` | `FairlightAudioMixerSolo` | 0x20
//! `FAOC` | `CapabilitiesFairlightAudioMixerAuxOut` | 0xc
//! `FASD` | `FairlightAudioMixerInputSourceDeactivated` | 0x18
//! `FASG` | `FairlightAudioMixerInputSourceInputGainProperties` | 0x20
//! `FASP` | `FairlightAudioMixerInputSourceProperties` | 0x3c
//! `FDLv` | `FairlightAudioMixerMasterOutLevels` | 0x24
//! `FIEP` | `FairlightAudioMixerAnalogInputExtendedProperties` | 0xc
//! `FMAI` | `FairlightAudioMixerAuxOutInputProperties` | 0x14
//! `FMAO` | `FairlightAudioMixerAuxOutMixerProperties` | 0x14
//! `FMHP` | `FairlightAudioMixerHeadphoneOutProperties` | 0x28
//! `FMLv` | `FairlightAudioMixerInputSourceLevels` | 0x30
//! `MOCP` | `FairlightAudioMixerMasterOutCompressorProperties` | 0x20
//! `RFIP` | `ResetFairlightAudioMixerInputSourceLevelPeaks` | 0x1c
//! `RFLP` | `ResetFairlightAudioMixerLevelPeaks` | 0xc
//! `RICD` | `ResetFairlightAudioMixerInputSourceDynamics` | 0x1c
//! `RICE` | `ResetFairlightAudioMixerInputSourceEqualiser` | 0x1c
//! `RMOD` | `ResetFairlightAudioMixerMasterOutDynamics` | 0xc
//! `RMOE` | `ResetFairlightAudioMixerMasterOutEqualiser` | 0xc
//! `SFLN` | `SetFairlightAudioMixerLevelsNotification` | 0xc

use binrw::binrw;

#[binrw]
#[brw(repr = u8)]
#[derive(Debug, FromPrimitive, ToPrimitive, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum FairlightAudioInputSourceState {
    /// Audio source is muted.
    Off = 0x01,
    /// Audio source is enabled.
    On = 0x02,
    /// Audio source is used when the video source is active.
    AudioFollowsVideo = 0x04,
}

/// `FASP`: Fairlight Audio mixer input Source Properties
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FairlightAudioMixerInputSourceProperties {
    /// Source ID
    #[brw(pad_size_to = 8)]
    pub source_id: u16,

    unknown_10: u64,
    #[brw(pad_size_to = 4)]
    unknown_18: u16,
    unknown_1c: u32,

    #[brw(pad_size_to = 2)]
    unknown_20: u8,
    unknown_22: u16,

    #[brw(pad_size_to = 4)]
    unknown_24: u16,

    unknown_28: u32,
    unknown_2c: u32,

    /// Pan, in 0.01dB. Left is negative, right is positive.
    #[brw(pad_size_to = 4)]
    pub pan: i16,

    /// Level, in 0.01dB
    pub level: i32,

    unknown_38: u8,
    #[brw(pad_size_to = 3)]
    pub state: FairlightAudioInputSourceState,
}

/// `FMTl`: Fairlight audio mixer tally (`FairlightAudioMixerTally`)
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FairlightAudioMixerTally {
    #[brw(pad_after = 6)]
    #[br(temp)]
    #[bw(try_calc(u16::try_from(entries.len())))]
    length: u16,

    #[br(count = length)]
    pub entries: Vec<FairlightAudioMixerTallyEntry>,
}

/// `BEPStructFairlightAudioSourceTally`
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FairlightAudioMixerTallyEntry {
    unknown_0: u64,

    pub source_id: u16,

    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub active: bool,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{atom::Atom, Result};
    use binrw::BinRead;
    use std::io::Cursor;

    #[test]
    fn fasp() -> Result<()> {
        // Off, Input 1, centre, level +10dB
        let cmd = hex::decode("003c0000464153500001000000000001ffffffffffff010001000000000000000002000006010004000000000000000000000bc2000003e807010027")?;
        let cmd = Atom::read(&mut Cursor::new(&cmd))?;

        let expected = Atom::new(FairlightAudioMixerInputSourceProperties {
            source_id: 0x1,
            unknown_10: 0xffffffffffff0100,
            unknown_18: 0x100,
            unknown_1c: 0,
            unknown_20: 0,
            unknown_22: 0,
            unknown_24: 0x601,
            unknown_28: 0,
            unknown_2c: 0,
            pan: 0,
            level: 1000,
            unknown_38: 0x7,
            state: FairlightAudioInputSourceState::Off,
        });
        assert_eq!(expected, cmd);

        // Off, Input 2, pan left -10dB, level -5dB
        let cmd = hex::decode("003c0000464153500002000000000001ffffffffffff0100010000000000000000020000060100040000000000000000fc180bc2fffffe0c07010027")?;
        let cmd = Atom::read(&mut Cursor::new(&cmd))?;

        let expected = Atom::new(FairlightAudioMixerInputSourceProperties {
            source_id: 0x2,
            unknown_10: 0xffffffffffff0100,
            unknown_18: 0x100,
            unknown_1c: 0,
            unknown_20: 0,
            unknown_22: 0,
            unknown_24: 0x601,
            unknown_28: 0,
            unknown_2c: 0,
            pan: -1000,
            level: -500,
            unknown_38: 0x7,
            state: FairlightAudioInputSourceState::Off,
        });
        assert_eq!(expected, cmd);

        // On, Input 1, 0dB, centred
        let cmd = hex::decode("003c0000464153500001000000000001ffffffffffff010001000000000000000002000006010004000000000000000000000bc20000000007020027")?;
        let cmd = Atom::read(&mut Cursor::new(&cmd))?;

        let expected = Atom::new(FairlightAudioMixerInputSourceProperties {
            source_id: 0x1,
            unknown_10: 0xffffffffffff0100,
            unknown_18: 0x100,
            unknown_1c: 0,
            unknown_20: 0,
            unknown_22: 0,
            unknown_24: 0x601,
            unknown_28: 0,
            unknown_2c: 0,
            pan: 0,
            level: 0,
            unknown_38: 0x7,
            state: FairlightAudioInputSourceState::On,
        });
        assert_eq!(expected, cmd);

        // Off, Input 1, 0dB, centred
        let cmd = hex::decode("003c0000464153500001000000000001ffffffffffff010001000000000000000002000006010004000000000000000000000bc20000000007010027")?;
        let cmd = Atom::read(&mut Cursor::new(&cmd))?;

        let expected = Atom::new(FairlightAudioMixerInputSourceProperties {
            source_id: 0x1,
            unknown_10: 0xffffffffffff0100,
            unknown_18: 0x100,
            unknown_1c: 0,
            unknown_20: 0,
            unknown_22: 0,
            unknown_24: 0x601,
            unknown_28: 0,
            unknown_2c: 0,
            pan: 0,
            level: 0,
            unknown_38: 0x7,
            state: FairlightAudioInputSourceState::Off,
        });

        assert_eq!(expected, cmd);

        // AFV, Input 1, 0dB, centred
        let cmd = hex::decode("003c0000464153500001000000000001ffffffffffff010001000000000000000002000006010004000000000000000000000bc20000000007040027")?;
        let cmd = Atom::read(&mut Cursor::new(&cmd))?;

        let expected = Atom::new(FairlightAudioMixerInputSourceProperties {
            source_id: 0x1,
            unknown_10: 0xffffffffffff0100,
            unknown_18: 0x100,
            unknown_1c: 0,
            unknown_20: 0,
            unknown_22: 0,
            unknown_24: 0x601,
            unknown_28: 0,
            unknown_2c: 0,
            pan: 0,
            level: 0,
            unknown_38: 0x7,
            state: FairlightAudioInputSourceState::AudioFollowsVideo,
        });

        assert_eq!(expected, cmd);
        Ok(())
    }

    #[test]
    fn fmtl() -> Result<()> {
        let cmd = hex::decode("00540000464d546c000600238d00238effffffffffff0100000101ffffffffffff0100000200ffffffffffff0100000300ffffffffffff0100000400ffffffffffff0100051500ffffffffffff01000516000400")?;
        let cmd = Atom::read(&mut Cursor::new(&cmd))?;

        let expected = Atom::new(FairlightAudioMixerTally {
            entries: vec![
                FairlightAudioMixerTallyEntry {
                    unknown_0: 0xffffffffffff0100,
                    source_id: 0x1,
                    active: true,
                },
                FairlightAudioMixerTallyEntry {
                    unknown_0: 0xffffffffffff0100,
                    source_id: 0x2,
                    active: false,
                },
                FairlightAudioMixerTallyEntry {
                    unknown_0: 0xffffffffffff0100,
                    source_id: 0x3,
                    active: false,
                },
                FairlightAudioMixerTallyEntry {
                    unknown_0: 0xffffffffffff0100,
                    source_id: 0x4,
                    active: false,
                },
                FairlightAudioMixerTallyEntry {
                    unknown_0: 0xffffffffffff0100,
                    source_id: 0x515,
                    active: false,
                },
                FairlightAudioMixerTallyEntry {
                    unknown_0: 0xffffffffffff0100,
                    source_id: 0x516,
                    active: false,
                },
            ],
        });

        assert_eq!(expected, cmd);

        Ok(())
    }
}
