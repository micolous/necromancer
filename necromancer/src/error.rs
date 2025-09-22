use std::io::Error as IoError;
use thiserror::Error;

/// Error types
#[derive(Debug, Error)]
pub enum Error {
    #[cfg(test)]
    #[error(transparent)]
    FromHexError(#[from] hex::FromHexError),

    #[error(transparent)]
    IoError(#[from] IoError),

    #[error(transparent)]
    Protocol(#[from] crate::protocol::Error),

    #[error("data parse error: {0}")]
    BinRwError(#[from] binrw::Error),

    #[error("channel unavailable, likely dropped")]
    ChannelUnavailable,

    #[error("internal error")]
    Internal,

    #[error("timeout waiting for response")]
    Timeout,

    #[error("unknown parameter")]
    UnknownParameter,

    #[error("parameter out of valid range")]
    ParameterOutOfRange,

    #[error("invalid length")]
    InvalidLength,

    #[error("switcher does not support the requested feature")]
    FeatureUnavailable,

    #[error("disconnected")]
    Disconnected,

    #[error("not found")]
    NotFound,

    #[error("mixer overloaded")]
    MixerOverloaded,

    #[error("unexpected state")]
    UnexpectedState,

    #[error("switcher reported transfer error: {0}")]
    SwitcherTransferError(u8),
}
