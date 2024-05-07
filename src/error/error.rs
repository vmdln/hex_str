use std::borrow::Borrow;

use super::hex_error::HexError;

#[derive(Debug, thiserror::Error)]
#[error("{inner}")]
pub struct Error<T, E>
where
    E: HexError + Clone + std::error::Error,
{
    inner: T,
    kind: E,
}

impl<T, E> Error<T, E>
where
    E: HexError + Clone + std::error::Error,
{
    pub fn new(inner: T, kind: E) -> Self {
        Self { inner, kind }
    }

    pub fn kind(&self) -> E {
        self.kind.clone()
    }

    pub fn inner<B>(&self) -> &B
    where
        T: Borrow<B>,
    {
        self.inner.borrow()
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}
