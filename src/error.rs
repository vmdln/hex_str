#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("invalid input length")]
    InvalidLength,
    #[error("non-hex character encountered")]
    InvalidCharacter,
}
