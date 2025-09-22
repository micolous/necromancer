#![doc = include_str!("../README.md")]

#[macro_use]
extern crate num_derive;

#[macro_use]
extern crate tracing;

pub mod atom;
mod error;
mod packet;
mod rle;
mod util;

pub use crate::{
    atom::Atom,
    error::Error,
    packet::{AtemControl, AtemPacket, AtemPacketFlags},
    rle::{rle_size_elements, RleCompressor, RleDecompressor, RLE_MARKER},
};

/// Result type.
pub type Result<T = ()> = std::result::Result<T, Error>;
