pub(crate) mod sealed {
    pub trait Sealed {}
}
pub trait HexError: sealed::Sealed {}
