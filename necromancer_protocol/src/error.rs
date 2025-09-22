use crate::atom::Version;
use std::str::Utf8Error;
use thiserror::Error;

/// Error types.
#[derive(Debug, Error)]
pub enum Error {
    #[cfg(test)]
    #[error(transparent)]
    FromHexError(#[from] hex::FromHexError),

    #[error(transparent)]
    Utf8(#[from] Utf8Error),

    #[error("unexpected state")]
    UnexpectedState,

    #[error("invalid length")]
    InvalidLength,

    #[error("parameter out of valid range")]
    ParameterOutOfRange,

    #[error("drop frame timecodes are not supported")]
    DropFrame,

    #[error("data parse error: {0}")]
    BinRwError(#[from] binrw::Error),

    #[error("unsupported firmware version: {0:?}")]
    UnsupportedFirmwareVersion(Version),
}
