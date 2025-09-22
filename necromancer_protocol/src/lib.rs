#![doc = include_str!("../README.md")]

#[macro_use]
extern crate num_derive;

#[macro_use]
extern crate tracing;

pub mod atom;
pub mod ay10;
mod error;
mod packet;
pub mod rle;
pub mod structs;
mod util;

pub use crate::{
    atom::Atom,
    error::Error,
    packet::{AtemControl, AtemPacket, AtemPacketFlags},
    util::IntReader,
};

/// Result type.
pub type Result<T = ()> = std::result::Result<T, Error>;
