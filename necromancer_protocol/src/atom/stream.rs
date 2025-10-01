//! RTMP streaming commands
//!
//! ## Unimplemented atoms
//!
//! FourCC | Atom name | Length
//! ------ | --------- | ------
//! `StRS` | `StreamRtmpStatus` | 0xc
//! `StrR` | `StreamRTMP` | 0xc
//! `STAB` | `StreamRtmpAudioBitrates` | 0x10
//! `SAth` | `StreamRtmpAuthentication` | 0x88
//! `SRDR` | `StreamRtmpDurationRequest` | 0x8
//! `SLow` | `StreamRtmpLowLatency` | 0xc
//! `SRSU` | `StreamRtmpSetup` | 0x450
//! `SRES` | `StreamRtmpSrtExtensions` | 0x20c
//! `SCPB` | `StreamRtmpStreamingCapabilities` | 0xc
//! `SRSD` | `StreamRtmpStreamingDuration` | 0x10
//! `SRSS` | `StreamRtmpStreamingStatistics` | 0x10
//! `SRST` | `StreamRtmpStreamingTimecode` | 0x10
//! `SCTR` | `StreamingControl` | 0x10
//! `SFPr` | `StreamingProfile` | 0x60
//! `SSDC` | `StreamDownConvertMode` | 0xc
