/// An error that may occur when parsing hex strings
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum HexArrayError {
    /// The input didn't have required length
    #[error("invalid input length, expected `{expected}`, encountered: `{encountered}`")]
    InvalidLength { expected: usize, encountered: usize },
    /// The input contained invalid character
    #[error("invalid byte `{msb:02x}{lsb:02x}` encountered at index {index}")]
    InvalidByte { msb: u8, lsb: u8, index: usize },
}

/// An error that may occur when parsing hex strings
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum HexVectorError {
    /// The input didn't have required length
    #[error("non-even input length, encountered: `{encountered}`")]
    InvalidLength { encountered: usize },
    /// The input contained invalid character
    #[error("invalid byte `{msb:02x}{lsb:02x}` encountered at index {index}")]
    InvalidByte { msb: u8, lsb: u8, index: usize },
}
