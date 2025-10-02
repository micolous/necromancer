//! # RTMP streaming; 0/16 atoms
//!
//! ## Unimplemented atoms (16)
//!
//! FourCC | Atom name | Length
//! ------ | --------- | ------
//! `CRSS` | `ChangeStreamRtmpSetup` | 0x454
//! `SAth` | `StreamRtmpAuthentication` | 0x88
//! `SCPB` | `StreamRtmpStreamingCapabilities` | 0xc
//! `SCTR` | `StreamingControl` | 0x10
//! `SFPr` | `StreamingProfile` | 0x60
//! `SLow` | `StreamRtmpLowLatency` | 0xc
//! `SRDR` | `StreamRtmpDurationRequest` | 0x8
//! `SRES` | `StreamRtmpSrtExtensions` | 0x20c
//! `SRSD` | `StreamRtmpStreamingDuration` | 0x10
//! `SRSS` | `StreamRtmpStreamingStatistics` | 0x10
//! `SRST` | `StreamRtmpStreamingTimecode` | 0x10
//! `SRSU` | `StreamRtmpSetup` | 0x450
//! `SSDC` | `StreamDownConvertMode` | 0xc
//! `STAB` | `StreamRtmpAudioBitrates` | 0x10
//! `StrR` | `StreamRTMP` | 0xc
//! `StRS` | `StreamRtmpStatus` | 0xc
