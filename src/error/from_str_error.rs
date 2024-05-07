use super::{hex_error::sealed::Sealed, HexError};

#[derive(Debug, Clone, thiserror::Error)]
pub enum FromStrError {
    #[error("invalid input length")]
    InvalidLength,
    #[error("non-hex character encountered")]
    InvalidCharacter,
}

impl Sealed for FromStrError {}
impl HexError for FromStrError {}
