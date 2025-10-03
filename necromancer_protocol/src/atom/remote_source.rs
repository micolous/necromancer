//! # Remote source; 1/11 atoms
//!
//! ## Unimplemented atoms (10)
//!
//! FourCC | Atom name | Length
//! ------ | --------- | ------
//! `RIGx` | `RemoteSourceExternalGenerateKey` | 0xc
//! `RILo` | `RemoteSource` | 0x90
//! `RIMs` | `RemoteSourceExternal` | 0x8c
//! `RIMa` | `RemoteSourceExternalAdded` | 0x8c
//! `RIMd` | `RemoteSourceExternalRemove` | 0xc
//! `RISs` | `RemoteSourceSettings` | 0x10
//! `RSca` | `RemoteSourceCapabilities` | 0xc
//! `RSDs` | `RemoteSourceDiscoverable` | 0xc
//! `RSis` | `RemoteSourceInternetSettings` | 0x10c
//! `RSpr` | `RemoteSourceInternetProbeStatus` | 0xc
//! `RXML` | `RemoteSourceExternalXML` | 0x40c

use binrw::binrw;

/// `RSip`: Remote source force internet probe (`RemoteSourceForceInternetProbe`)
///
/// ## Packet format
///
/// No payload.
#[binrw]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct RemoteSourceForceInternetProbe {}

/// Command to request RTMP streaming duration.
pub const REMOTE_SOURCE_FORCE_INTERNET_PROBE: RemoteSourceForceInternetProbe =
    RemoteSourceForceInternetProbe {};
