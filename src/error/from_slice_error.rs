use super::hex_error::{sealed::Sealed, HexError};

#[derive(Debug, Clone, thiserror::Error)]
#[error("invalid input length")]
pub struct FromSliceError;

impl Sealed for FromSliceError {}
impl HexError for FromSliceError {}
