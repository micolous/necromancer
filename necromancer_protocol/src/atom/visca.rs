//! # Visca PTZ camera control; 0/17 atoms
//!
//! ## Unimplemented atoms
//!
//! FourCC | Atom name | Length
//! ------ | --------- | ------
//! `CPZS` | `PtzRs422ViscaChangeSettings` | 0x10
//! `InVs` | `InputViscaDevice` | 0x10
//! `PZCS` | `PtzRs422ViscaConnectedState` | 0xc
//! `PZGP` | `PtzRs422ViscaGotoPanTiltPosition` | 0x10
//! `PZGZ` | `PtzRs422ViscaGotoZoomPosition` | 0xc
//! `PZPC` | `PtzRs422ViscaPosition` | 0x10
//! `PZSA` | `PtzRs422ViscaAutoAllocateAddresses` | 0x8
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
