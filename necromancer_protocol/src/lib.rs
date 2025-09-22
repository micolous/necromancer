#![doc = include_str!("../README.md")]

#[macro_use]
extern crate num_derive;

#[macro_use]
extern crate tracing;

pub mod atom;
mod error;
mod packet;
mod util;

pub use crate::{
    atom::Atom,
    error::Error,
    packet::{AtemControl, AtemPacket, AtemPacketFlags},
};

/// Result type.
pub type Result<T = ()> = std::result::Result<T, Error>;
