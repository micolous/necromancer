//! # RTMP streaming; 1/16 atoms
//!
//! ## Unimplemented atoms (15)
//!
//! FourCC | Atom name | Length
//! ------ | --------- | ------
//! `CRSS` | `ChangeStreamRtmpSetup` | 0x454
//! `SAth` | `StreamRtmpAuthentication` | 0x88
//! `SCPB` | `StreamRtmpStreamingCapabilities` | 0xc
//! `SCTR` | `StreamingControl` | 0x10
//! `SFPr` | `StreamingProfile` | 0x60
//! `SLow` | `StreamRtmpLowLatency` | 0xc
//! `SRES` | `StreamRtmpSrtExtensions` | 0x20c
//! `SRSD` | `StreamRtmpStreamingDuration` | 0x10
//! `SRSS` | `StreamRtmpStreamingStatistics` | 0x10
//! `SRST` | `StreamRtmpStreamingTimecode` | 0x10
//! `SRSU` | `StreamRtmpSetup` | 0x450
//! `SSDC` | `StreamDownConvertMode` | 0xc
//! `STAB` | `StreamRtmpAudioBitrates` | 0x10
//! `StrR` | `StreamRTMP` | 0xc
//! `StRS` | `StreamRtmpStatus` | 0xc

use binrw::binrw;

/// `SRDR`: RTMP stream duration request (`StreamRtmpDurationRequest`)
///
/// ## Packet format
///
/// No payload.
#[binrw]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct RtmpDurationRequest {}

/// Command to request RTMP streaming duration.
pub const RTMP_DURATION_REQUEST: RtmpDurationRequest = RtmpDurationRequest {};
