/// An error that may occur when parsing hex strings
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// The input didn't have required length
    #[error("invalid input length, expected `{expected}`, encountered: `{encountered}`")]
    InvalidLength { expected: usize, encountered: usize },
    /// The input contained invalid character
    #[error("invalid character `{v:02x}` encountered at index {index}")]
    InvalidCharacter { v: u8, index: usize },
}
