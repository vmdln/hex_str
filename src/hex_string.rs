use std::{
    borrow::Cow,
    fmt::{Debug, Display, LowerHex, UpperHex},
    str::FromStr,
};

use super::{
    error::{Error, FromStrError},
    fmt::{Lower, Upper},
    utils,
};

#[derive(Clone, PartialEq, Eq)]
pub struct HexString(Vec<u8>);

impl HexString {
    #[must_use]
    pub fn new(v: impl Into<Vec<u8>>) -> Self {
        Self(v.into())
    }

    #[must_use]
    pub fn inner(&self) -> &[u8] {
        &self.0
    }

    #[must_use]
    pub fn inner_mut(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }

    #[must_use]
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }

    #[must_use]
    pub fn as_lower(&self) -> Lower<'_> {
        Lower(Cow::Borrowed(&self.0))
    }

    #[must_use]
    pub fn to_lower(&self) -> Lower<'static> {
        Lower(Cow::Owned(self.0.clone()))
    }

    #[must_use]
    pub fn into_lower(self) -> Lower<'static> {
        Lower(Cow::Owned(self.0))
    }

    #[must_use]
    pub fn as_upper(&self) -> Upper<'_> {
        Upper(Cow::Borrowed(&self.0))
    }

    #[must_use]
    pub fn to_upper(&self) -> Upper<'static> {
        Upper(Cow::Owned(self.0.clone()))
    }

    #[must_use]
    pub fn into_upper(self) -> Upper<'static> {
        Upper(Cow::Owned(self.0))
    }
}

impl<const N: usize> From<&[u8; N]> for HexString {
    fn from(value: &[u8; N]) -> Self {
        Self::new(*value)
    }
}

impl From<&[u8]> for HexString {
    fn from(value: &[u8]) -> Self {
        Self::new(value)
    }
}

impl From<Vec<u8>> for HexString {
    fn from(value: Vec<u8>) -> Self {
        Self::new(value)
    }
}

impl From<HexString> for Vec<u8> {
    fn from(value: HexString) -> Self {
        value.0
    }
}

impl From<&HexString> for String {
    fn from(value: &HexString) -> Self {
        value.to_string()
    }
}

impl From<&HexString> for Cow<'static, str> {
    fn from(value: &HexString) -> Self {
        Self::Owned(value.to_string())
    }
}

impl<'a> TryFrom<&'a str> for HexString {
    type Error = Error<&'a str, FromStrError>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut bytes = value.bytes();
        let mut buf = Vec::new();

        while let Some(a) = bytes.next() {
            let b = bytes
                .next()
                .ok_or(Error::new(value, FromStrError::InvalidLength))?;

            buf.push(
                utils::from_hex([a, b]).ok_or(Error::new(value, FromStrError::InvalidCharacter))?,
            );
        }

        Ok(Self(buf))
    }
}

impl TryFrom<String> for HexString {
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

impl FromStr for HexString {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s).map_err(|err| err.kind())
    }
}

impl FromIterator<u8> for HexString {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl PartialEq<[u8]> for HexString {
    fn eq(&self, other: &[u8]) -> bool {
        self.0 == other
    }
}

impl PartialEq<&[u8]> for HexString {
    fn eq(&self, other: &&[u8]) -> bool {
        self.0 == *other
    }
}

impl<const N: usize> PartialEq<[u8; N]> for HexString {
    fn eq(&self, other: &[u8; N]) -> bool {
        self.0 == other
    }
}

impl<const N: usize> PartialEq<&[u8; N]> for HexString {
    fn eq(&self, other: &&[u8; N]) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Vec<u8>> for HexString {
    fn eq(&self, other: &Vec<u8>) -> bool {
        &self.0 == other
    }
}

impl PartialEq<str> for HexString {
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

impl PartialEq<&str> for HexString {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl Display for HexString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_lower().fmt(f)
    }
}

impl LowerHex for HexString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_lower().fmt(f)
    }
}

impl UpperHex for HexString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_upper().fmt(f)
    }
}

impl Debug for HexString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HexString")
            .field("inner", &self.to_string())
            .finish()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for HexString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct HexStringVisitor;

        impl<'de> serde::de::Visitor<'de> for HexStringVisitor {
            type Value = HexString;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_fmt(format_args!("hex string"))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Self::Value::try_from(v).map_err(|err| E::custom(err))
            }
        }

        deserializer.deserialize_str(HexStringVisitor)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for HexString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}
