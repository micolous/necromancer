//! # Audio (non-Fairlight); 0/23 atoms
//!
//! ## Unimplemented atoms (23)
//!
//! FourCC | Atom name | Length
//! ------ | --------- | ------
//! `_AMC` | `CapabilitiesAudioMixer` | 0xc
//! `AMHP` | `AudioMixerHeadphoneOutProperties` | 0x10
//! `AMIP` | `AudioMixerInputProperties` | 0x18
//! `AMLv` | `AudioMixerLevels` | (variable)
//! `AMMO` | `AudioMixerMasterOutProperties` | 0x10
//! `AMmO` | `AudioMixerMonitorOutProperties` | 0x14
//! `AMPP` | `AudioMixerProperties` | 0xc
//! `AMTl` | `AudioMixerTally` | (0xa + (audio_tally_len * 3) bytes)
//! `AROC` | `ChangeAudioRoutingOutputProperties` | 0x54
//! `AROP` | `AudioRoutingOutputProperties` | 0x58
//! `AROP` | `AudioRoutingOutputProperties` | 0x58
//! `ARSC` | `ChangeAudioRoutingSourceProperties` | 0x50
//! `ARSP` | `AudioRoutingSourceProperties` | 0x54
//! `ARSP` | `AudioRoutingSourceProperties` | 0x58
//! `CAMH` | `ChangeAudioMixerHeadphoneOutProperties` | 0x14
//! `CAMI` | `ChangeAudioMixerInputProperties` | 0x14
//! `CAMM` | `ChangeAudioMixerMasterOutProperties` | 0x10
//! `CAMm` | `ChangeAudioMixerMonitorOutProperties` | 0x14
//! `CAMP` | `ChangeAudioMixerProperties` | 0xc
//! `CMMP` | `ChangeMixMinusOutProperties` | 0x10
//! `MMOP` | `MixMinusOutProperties` | 0x14
//! `SALN` | `SetAudioMixerLevelsNotification` | 0xc
//! `RAMP` | `ResetAudioMixerPeakLevels` | 0x10
