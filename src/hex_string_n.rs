use std::{
    borrow::Cow,
    fmt::{Debug, Display, LowerHex, UpperHex},
    str::FromStr,
};

use super::{
    error::{Error, FromSliceError, FromStrError},
    fmt::{Lower, Upper},
    utils,
};

#[derive(Clone, PartialEq, Eq)]
pub struct HexStringN<const N: usize>([u8; N]);

impl<const N: usize> HexStringN<N> {
    #[must_use]
    pub fn new(v: impl Into<[u8; N]>) -> Self {
        Self(v.into())
    }

    #[must_use]
    pub fn inner(&self) -> &[u8; N] {
        &self.0
    }

    #[must_use]
    pub fn inner_mut(&mut self) -> &mut [u8; N] {
        &mut self.0
    }

    #[must_use]
    pub fn into_inner(self) -> [u8; N] {
        self.0
    }

    #[must_use]
    pub fn as_lower(&self) -> Lower<'_> {
        Lower(Cow::Borrowed(&self.0))
    }

    #[must_use]
    pub fn to_lower(&self) -> Lower<'static> {
        Lower(Cow::Owned(self.0.to_vec()))
    }

    #[must_use]
    pub fn as_upper(&self) -> Upper<'_> {
        Upper(Cow::Borrowed(&self.0))
    }

    #[must_use]
    pub fn to_upper(&self) -> Upper<'static> {
        Upper(Cow::Owned(self.0.to_vec()))
    }
}

impl<const N: usize> From<[u8; N]> for HexStringN<N> {
    fn from(value: [u8; N]) -> Self {
        Self::new(value)
    }
}

impl<const N: usize> From<HexStringN<N>> for [u8; N] {
    fn from(value: HexStringN<N>) -> Self {
        value.0
    }
}

impl<const N: usize> From<HexStringN<N>> for String {
    fn from(value: HexStringN<N>) -> Self {
        value.to_string()
    }
}

impl<const N: usize> From<HexStringN<N>> for Cow<'static, str> {
    fn from(value: HexStringN<N>) -> Self {
        Self::Owned(value.to_string())
    }
}

impl<'a, const N: usize> TryFrom<&'a [u8]> for HexStringN<N> {
    type Error = Error<&'a [u8], FromSliceError>;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        value
            .try_into()
            .map(Self)
            .map_err(|_| Error::new(value, FromSliceError))
    }
}

impl<const N: usize> TryFrom<Vec<u8>> for HexStringN<N> {
    type Error = Error<Vec<u8>, FromSliceError>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        match Self::try_from(&*value) {
            Ok(v) => Ok(v),
            Err(err) => {
                let kind = err.kind();
                Err(Error::new(value, kind))
            }
        }
    }
}

impl<'a, const N: usize> TryFrom<Cow<'a, [u8]>> for HexStringN<N> {
    type Error = Error<Cow<'a, [u8]>, FromSliceError>;

    fn try_from(value: Cow<'a, [u8]>) -> Result<Self, Self::Error> {
        match Self::try_from(&*value) {
            Ok(v) => Ok(v),
            Err(err) => {
                let kind = err.kind();
                Err(Error::new(value, kind))
            }
        }
    }
}

impl<'a, const N: usize> TryFrom<&'a str> for HexStringN<N> {
    type Error = Error<&'a str, FromStrError>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut bytes = value.bytes();
        let mut buf = [0; N];

        for v in &mut buf {
            let a = bytes
                .next()
                .ok_or(Error::new(value, FromStrError::InvalidLength))?;
            let b = bytes
                .next()
                .ok_or(Error::new(value, FromStrError::InvalidLength))?;

            *v =
                utils::from_hex([a, b]).ok_or(Error::new(value, FromStrError::InvalidCharacter))?;
        }

        if bytes.next().is_some() {
            Err(Error::new(value, FromStrError::InvalidLength))
        } else {
            Ok(Self(buf))
        }
    }
}

impl<const N: usize> TryFrom<String> for HexStringN<N> {
    type Error = Error<String, FromStrError>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match Self::try_from(value.as_str()) {
            Ok(v) => Ok(v),
            Err(err) => {
                let kind = err.kind();
                Err(Error::new(value, kind))
            }
        }
    }
}

impl<'a, const N: usize> TryFrom<Cow<'a, str>> for HexStringN<N> {
    type Error = Error<Cow<'a, str>, FromStrError>;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        match Self::try_from(&*value) {
            Ok(v) => Ok(v),
            Err(err) => {
                let kind = err.kind();
                Err(Error::new(value, kind))
            }
        }
    }
}

impl<const N: usize> FromStr for HexStringN<N> {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s).map_err(|err| err.kind())
    }
}

impl<const N: usize> PartialEq<[u8]> for HexStringN<N> {
    fn eq(&self, other: &[u8]) -> bool {
        self.0 == other
    }
}

impl<const N: usize> PartialEq<&[u8]> for HexStringN<N> {
    fn eq(&self, other: &&[u8]) -> bool {
        self.0 == *other
    }
}

impl<const N: usize> PartialEq<[u8; N]> for HexStringN<N> {
    fn eq(&self, other: &[u8; N]) -> bool {
        &self.0 == other
    }
}

impl<const N: usize> PartialEq<&[u8; N]> for HexStringN<N> {
    fn eq(&self, other: &&[u8; N]) -> bool {
        &self.0 == *other
    }
}

impl<const N: usize> PartialEq<Vec<u8>> for HexStringN<N> {
    fn eq(&self, other: &Vec<u8>) -> bool {
        other == &self.0
    }
}

impl<const N: usize> PartialEq<Cow<'_, [u8]>> for HexStringN<N> {
    fn eq(&self, other: &Cow<'_, [u8]>) -> bool {
        self.0 == **other
    }
}

impl<const N: usize> PartialEq<str> for HexStringN<N> {
    fn eq(&self, other: &str) -> bool {
        let mut other = other.bytes();

        for v in &self.0 {
            let Some(a) = other.next() else {
                return false;
            };
            let Some(b) = other.next() else {
                return false;
            };

            if let Some(w) = utils::from_hex([a, b]) {
                if *v != w {
                    return false;
                }
            } else {
                return false;
            }
        }

        other.next().is_none()
    }
}

impl<const N: usize> PartialEq<&str> for HexStringN<N> {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl<const N: usize> PartialEq<Cow<'_, str>> for HexStringN<N> {
    fn eq(&self, other: &Cow<'_, str>) -> bool {
        self == &**other
    }
}

impl<const N: usize> Display for HexStringN<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_lower().fmt(f)
    }
}

impl<const N: usize> LowerHex for HexStringN<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_lower().fmt(f)
    }
}

impl<const N: usize> UpperHex for HexStringN<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_upper().fmt(f)
    }
}

impl<const N: usize> Debug for HexStringN<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HexStringN")
            .field("n", &N)
            .field("inner", &self.to_string())
            .finish()
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize> serde::Deserialize<'de> for HexStringN<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct HexStringVisitor<const N: usize>;

        impl<'de, const N: usize> serde::de::Visitor<'de> for HexStringVisitor<N> {
            type Value = HexStringN<N>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_fmt(format_args!("hex string containing {N} bytes"))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Self::Value::try_from(v).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(HexStringVisitor)
    }
}

#[cfg(feature = "serde")]
impl<const N: usize> serde::Serialize for HexStringN<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}
