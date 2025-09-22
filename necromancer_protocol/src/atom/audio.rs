//! Audio (non-Fairlight) commands
//!
//! ## Unimplemented atoms
//!
//! FourCC | Atom name | Length
//! ------ | --------- | ------
//! `AROP` | `AudioRoutingOutputProperties` | (0x58 bytes)
//! `AMHP` | `AudioMixerHeadphoneOutProperties` | (0x10 bytes)
//! `AMIP` | `AudioMixerInputProperties` | (0x18 bytes)
//! `AMLv` | `AudioMixerLevels` | (variable)
//! `AMMO` | `AudioMixerMasterOutProperties` | (0x10 bytes)
//! `AMmO` | `AudioMixerMonitorOutProperties` | (0x14 bytes)
//! `AMPP` | `AudioMixerProperties` | (0xc bytes)
//! `AMTl` | `AudioMixerTally` | (0xa + (audio_tally_len * 3) bytes)
//! `ARSP` | `AudioRoutingSourceProperties` | (0x54 bytes)
//! `_AMC` | `CapabilitiesAudioMixer` | (0xc bytes)
