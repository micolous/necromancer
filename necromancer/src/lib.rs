#![doc = include_str!("../README.md")]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate tracing;

mod controller;
mod error;
mod rle;
mod state;
mod udp;

pub use {
    crate::{
        controller::AtemController,
        error::Error,
        state::{AtemState, StateUpdate},
        udp::AtemUdpChannel,
    },
    necromancer_protocol as protocol,
};
pub type Result<T = ()> = std::result::Result<T, Error>;
