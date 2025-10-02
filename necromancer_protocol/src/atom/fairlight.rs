//! # Fairlight audio; 6/52 atoms
//!
//! ## Unimplemented atoms (46)
//!
//! FourCC | Atom name | Length
//! ------ | --------- | ------
//! `AEBP` | `FairlightAudioMixerInputSourceEqualiserBandProperties` | 0x2c
//! `AICP` | `FairlightAudioMixerInputSourceCompressorProperties` | 0x30
//! `AILP` | `FairlightAudioMixerInputSourceLimiterProperties` | 0x2c
//! `AIXP` | `FairlightAudioMixerInputSourceExpanderProperties` | 0x30
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
//! `FMPP` | `FairlightAudioMixerProperties` | 0xc
//! `MOCP` | `FairlightAudioMixerMasterOutCompressorProperties` | 0x20
//! `RFIP` | `ResetFairlightAudioMixerInputSourceLevelPeaks` | 0x1c
//! `RFLP` | `ResetFairlightAudioMixerLevelPeaks` | 0xc
//! `RICD` | `ResetFairlightAudioMixerInputSourceDynamics` | 0x1c
//! `RICE` | `ResetFairlightAudioMixerInputSourceEqualiser` | 0x1c
//! `RMOD` | `ResetFairlightAudioMixerMasterOutDynamics` | 0xc
//! `RMOE` | `ResetFairlightAudioMixerMasterOutEqualiser` | 0xc
//! `SFLN` | `SetFairlightAudioMixerLevelsNotification` | 0xc

use crate::structs::{EqualiserRange, EqualiserShape, FairlightEqualiserBandRangeFrequencyLimits};
use binrw::{binrw, BinRead, BinWrite};
use modular_bitfield::{bitfield, specifiers::B12, Specifier};

/// `_FAC`: Fairlight audio mixer capabilities (`CapabilitiesFairlightAudioMixer`)
///
/// ## Packet format
///
/// * `u8`: input channel count
/// * `bool`: has headphone output
/// * 2 bytes padding
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CapabilitiesFairlightAudioMixer {
    pub channels: u8,

    #[brw(pad_after = 2)]
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub has_headphone_output: bool,
}

#[bitfield(bits = 16)]
#[repr(u16)]
#[derive(Specifier, BinRead, BinWrite, Debug, Default, PartialEq, Eq, Clone, Copy)]
#[br(map = From::<u16>::from)]
#[bw(map = |&x| Into::<u16>::into(x))]
pub struct HeadphoneOutputCapabilities {
    pub has_solo_output: bool,
    pub has_talkback: bool,
    pub has_sidetone: bool,
    pub has_mute: bool,
    #[skip]
    __: B12,
}

/// `_FMH`: Fairlight audio mixer headphone output capabilities
/// (`CapabilitiesFairlightAudioMixerHeadphoneOut`)
///
/// ## Packet format
///
/// * `u16`: [HeadphoneOutputCapabilities][] bitmask
/// * 2 bytes padding
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CapabilitiesFairlightAudioMixerHeadphoneOut {
    #[brw(pad_after = 2)]
    pub caps: HeadphoneOutputCapabilities,
}

/// `_FEC`: Fairlight audio mixer equaliser band range capabilities
/// (`CapabilitiesFairlightEqualiserBandRange`)
///
/// ## Packet format
///
/// * `u16`: entry count
/// * 2 bytes padding
/// * repeated `BEPStructFairlightEqualiserBandRangeFrequencyLimits` (0xc bytes)
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FairlightEqualiserBandRangeCapabilities {
    #[brw(pad_after = 2)]
    #[br(temp)]
    #[bw(try_calc(u16::try_from(v.len())))]
    length: u16,

    #[br(count=length)]
    v: Vec<FairlightEqualiserBandRangeFrequencyLimits>,
}

/// `AMBP`: Fairlight audio mixer master out equaliser band properties
/// (`FairlightAudioMixerMasterOutEqualiserBandProperties`)
///
/// ## Packet format
///
/// * `u8`: band ID
/// * `bool`: enabled
/// * `u8`: supported shapes (bitmask)
/// * `u8`: current shape
/// * `u8`: supported frequency ranges (bitmask)
/// * `u8`: current frequency range
/// * 2 bytes padding
/// * `u32`: frequency
/// * `i32`: gain
/// * `u16`: q factor
#[binrw]
#[brw(big)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FairlightAudioMixerMasterOutEqualiserBandProperties {
    pub band_id: u8,

    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub enabled: bool,

    pub supported_shapes: u8,
    pub shape: EqualiserShape,
    pub supported_frequency_ranges: u8,
    #[brw(pad_after = 2)]
    pub frequency_range: EqualiserRange,

    /// Frequency, in hertz
    pub frequency: u32,

    /// Band gain, in 0.01dB.
    pub gain: i32,

    /// Q Factor, in 0.01dB. Only valid for `shape` == [`BandPass`][EqualiserShape::BandPass].
    #[brw(pad_after = 2)]
    pub q_factor: u16,
}

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
    fn capabilities() -> Result {
        // ATEM Mini Pro, ATEM Mini Pro ISO
        let cmd = hex::decode("000c00005f46414306000000")?;
        let cmd = Atom::read(&mut Cursor::new(&cmd))?;

        let expected = Atom::new(CapabilitiesFairlightAudioMixer {
            channels: 6,
            has_headphone_output: false,
        });
        assert_eq!(expected, cmd);

        Ok(())
    }

    #[test]
    fn ambp() -> Result {
        let cmd = hex::decode("001c0000414d425001012d010f012d04000000310000000000640000")?;
        let cmd = Atom::read(&mut Cursor::new(&cmd))?;

        Ok(())
    }

    #[test]
    fn fasp() -> Result {
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
    fn fmtl() -> Result {
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
