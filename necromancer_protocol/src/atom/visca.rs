//! # Visca PTZ camera control; 1/17 atoms
//!
//! ## Unimplemented atoms (16)
//!
//! FourCC | Atom name | Length
//! ------ | --------- | ------
//! `CPZS` | `PtzRs422ViscaChangeSettings` | 0x10
//! `InVs` | `InputViscaDevice` | 0x10
//! `PZCS` | `PtzRs422ViscaConnectedState` | 0xc
//! `PZGP` | `PtzRs422ViscaGotoPanTiltPosition` | 0x10
//! `PZGZ` | `PtzRs422ViscaGotoZoomPosition` | 0xc
//! `PZPC` | `PtzRs422ViscaPosition` | 0x10
//! `PZVC` | `PtzRs422ViscaVelocity` | 0xc
//! `SPZS` | `PtzRs422ViscaSettings` | 0xc
//! `SPZV` | `PtzRs422ViscaSetVelocity` | 0x10
//! `vscp` | `ViscaCapabilities` | 0x14
//! `vscR` | `RemoveViscaDevice` | 0xc
//! `vsDP` | `ChangeViscaProperties` | 0x94
//! `vsDp` | `ViscaProperties` | 0x90
//! `vsIP` | `AddViscaIPDevice` | 0x88
//! `vsPG` | `ViscaIPAddressPing` | 0x48
//! `vspg` | `ViscaIPAddressPingResponse` | 0x4c

use binrw::binrw;

/// `PZSA`: Auto-allocate addresses to Visca-compatible PTZ cameras connected over RS-422
/// (`PtzRs422ViscaAutoAllocateAddresses`)
///
/// ## Packet format
///
/// No payload.
#[binrw]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Visca422AutoAllocateAddresses {}

/// Command to automatically allocate addresses to Visca-compatible PTZ cameras connected over
/// RS-422.
pub const VISCA_422_AUTO_ALLOCATE_ADDRESSES: Visca422AutoAllocateAddresses =
    Visca422AutoAllocateAddresses {};
