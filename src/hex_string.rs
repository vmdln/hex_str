use std::{
    borrow::{Borrow, BorrowMut},
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    str::FromStr,
};

use crate::{utils, HexStringError};

/// A hex string of variable length
///
/// For hex strings of constant length see [`HexStringN`](crate::HexStringN)
///
/// ## Example:
/// ```
/// use hex_str::HexString;
///
/// // byte arrays are always valid
/// let a = HexString::new([0x01, 0xde]);
/// assert_eq!(a, "01de");
/// assert_eq!(a, [0x01, 0xde]);
///
/// let b: HexString = "01de".parse().unwrap();
/// assert_eq!(a, b);
/// ```
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HexString(Vec<u8>);

impl HexString {
    /// Create a new `HexString`.
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexString;
    ///
    /// let v = HexString::new([0x1a, 0x2b, 0x3c, 0x4d]);
    /// assert_eq!(v, [0x1a, 0x2b, 0x3c, 0x4d]);
    /// assert_eq!(v, "1a2b3c4d");
    /// ```
    #[must_use]
    pub fn new(v: impl Into<Vec<u8>>) -> Self {
        Self(v.into())
    }

    /// Convert `self` to its string representation, lowercase.
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexString;
    ///
    /// let v: HexString = "1A2B3c4d".parse().unwrap();
    /// assert_eq!(v.to_lower(), "1a2b3c4d");
    /// ```
    #[must_use]
    pub fn to_lower(&self) -> String {
        self.0
            .iter()
            .copied()
            .flat_map(utils::to_hex_lower)
            .map(char::from)
            .collect()
    }

    /// Convert `self` to its string representation, uppercase.
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexString;
    ///
    /// let v: HexString = "1A2B3c4d".parse().unwrap();
    /// assert_eq!(v.to_upper(), "1A2B3C4D");
    /// ```
    #[must_use]
    pub fn to_upper(&self) -> String {
        self.0
            .iter()
            .copied()
            .flat_map(utils::to_hex_upper)
            .map(char::from)
            .collect()
    }

    /// Try to parse `bytes`, both lowercase and uppercase characters allowed.
    ///
    /// This is the same as using [`HexString::from_str`]/[`str::parse`] but
    /// accepts `impl AsRef<[u8]>`.
    ///
    /// # Errors
    /// - if `bytes.len() % 2 != 0`
    /// - if `bytes` contains characters other than `[0-9a-fA-F]`
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexString;
    ///
    /// let v = HexString::try_parse("1A2B3c4d");
    /// assert_eq!(v.unwrap(), "1a2b3c4d");
    /// ```
    pub fn try_parse(bytes: impl AsRef<[u8]>) -> Result<Self, HexStringError> {
        try_parse(bytes, utils::parse)
    }

    /// Try to parse `bytes`, only lowercase characters allowed.
    ///
    /// # Errors
    /// - if `bytes.len() % 2 != 0`
    /// - if `bytes` contains characters other than `[0-9a-f]`
    ///
    /// # Example:
    /// ```
    /// use hex_str::{HexString, HexStringError};
    ///
    /// let v = HexString::try_parse_lower("1a2b3c4d");
    /// assert_eq!(v.unwrap(), "1a2b3c4d");
    ///
    /// let v = HexString::try_parse_lower("1A2B3C4D");
    /// assert_eq!(v.unwrap_err(), HexStringError::InvalidByte { a: b'1', b: b'A', index: 0 });
    pub fn try_parse_lower(bytes: impl AsRef<[u8]>) -> Result<Self, HexStringError> {
        try_parse(bytes, utils::parse_lower)
    }

    /// Try to parse `bytes`, only uppercase characters allowed.
    ///
    /// # Errors
    /// - if `bytes.len() % 2 != 0`
    /// - if `bytes` contains characters other than `[0-9A-F]`
    ///
    /// # Example:
    /// ```
    /// use hex_str::{HexString, HexStringError};
    ///
    /// let v = HexString::try_parse_upper("1A2B3C4D");
    /// assert_eq!(v.unwrap(), "1a2b3c4d");
    ///
    /// let v = HexString::try_parse_upper("1a2b3c4d");
    /// assert_eq!(v.unwrap_err(), HexStringError::InvalidByte { a: b'1', b: b'a', index: 0 });
    pub fn try_parse_upper(bytes: impl AsRef<[u8]>) -> Result<Self, HexStringError> {
        try_parse(bytes, utils::parse_upper)
    }

    /// Return a mutable reference to the inner array.
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexString;
    ///
    /// let mut v = HexString::new([0x1a, 0x2b]);
    /// let mut_vec = v.as_mut_vec();
    ///
    /// mut_vec.push(0x3c);
    ///
    /// assert_eq!(v, "1a2b3c");
    /// ```
    #[must_use]
    pub fn as_mut_vec(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }
}

fn try_parse(
    bytes: impl AsRef<[u8]>,
    conversion_fn: impl Fn(u8, u8) -> Option<u8>,
) -> Result<HexString, HexStringError> {
    let bytes = bytes.as_ref();
    if bytes.len() % 2 != 0 {
        return Err(HexStringError::InvalidLength {
            encountered: bytes.len(),
        });
    }

    let mut ret = Vec::with_capacity(bytes.len() / 2);
    let mut i = 0;
    let mut j = 1;
    for _ in 0..ret.capacity() {
        let a = unsafe { *bytes.get_unchecked(i) };
        let b = unsafe { *bytes.get_unchecked(j) };
        conversion_fn(a, b)
            .ok_or(HexStringError::InvalidByte { a, b, index: i })
            .map(|w| ret.push(w))?;

        // if len == usize::MAX, this will overflow after the last iteration
        // which is fine
        i = i.wrapping_add(2);
        j = j.wrapping_add(2);
    }

    Ok(HexString::new(ret))
}

impl Display for HexString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.to_lower(), f)
    }
}

impl Debug for HexString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HexString")
            .field("inner", &self.to_string())
            .finish()
    }
}

impl FromStr for HexString {
    type Err = HexStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_parse(s)
    }
}

impl From<Vec<u8>> for HexString {
    fn from(value: Vec<u8>) -> Self {
        Self::new(value)
    }
}

impl From<&[u8]> for HexString {
    fn from(value: &[u8]) -> Self {
        Self::new(value)
    }
}

impl From<HexString> for Vec<u8> {
    fn from(value: HexString) -> Self {
        value.0
    }
}

impl TryFrom<&'_ str> for HexString {
    type Error = HexStringError;

    fn try_from(value: &'_ str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for HexString {
    type Error = HexStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<const N: usize> PartialEq<[u8; N]> for HexString {
    fn eq(&self, other: &[u8; N]) -> bool {
        &*self.0 == other
    }
}

impl<const N: usize> PartialEq<&[u8; N]> for HexString {
    fn eq(&self, other: &&[u8; N]) -> bool {
        &*self.0 == *other
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

impl PartialEq<str> for HexString {
    #[allow(clippy::many_single_char_names)]
    fn eq(&self, other: &str) -> bool {
        if other.len() % 2 == 0 && other.len() / 2 == self.0.len() {
            let bytes = other.as_bytes();
            let mut i = 0;
            let mut j = 1;
            for x in &*self.0 {
                let a = unsafe { *bytes.get_unchecked(i) };
                let b = unsafe { *bytes.get_unchecked(j) };

                let Some(y) = utils::parse(a, b) else {
                    return false;
                };

                if *x != y {
                    return false;
                }

                // if len == usize::MAX, this will overflow after the last iteration
                // which is fine
                i = i.wrapping_add(2);
                j = j.wrapping_add(2);
            }

            true
        } else {
            false
        }
    }
}

impl PartialEq<&str> for HexString {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<String> for HexString {
    fn eq(&self, other: &String) -> bool {
        self == other.as_str()
    }
}

impl Deref for HexString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HexString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<HexString> for HexString {
    fn as_ref(&self) -> &HexString {
        self
    }
}

impl AsMut<HexString> for HexString {
    fn as_mut(&mut self) -> &mut HexString {
        self
    }
}

impl AsRef<[u8]> for HexString {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for HexString {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl Borrow<Vec<u8>> for HexString {
    fn borrow(&self) -> &Vec<u8> {
        self
    }
}

impl BorrowMut<Vec<u8>> for HexString {
    fn borrow_mut(&mut self) -> &mut Vec<u8> {
        self
    }
}

impl Borrow<[u8]> for HexString {
    fn borrow(&self) -> &[u8] {
        self
    }
}

impl BorrowMut<[u8]> for HexString {
    fn borrow_mut(&mut self) -> &mut [u8] {
        self
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for HexString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HexString;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("hex string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse().map_err(|err| E::custom(err))
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(HexString::new(v))
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(HexString::new(v))
            }
        }

        deserializer.deserialize_str(Visitor)
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

#[cfg(all(test, feature = "rand"))]
mod tests {
    use rand::seq::SliceRandom;

    use super::HexString;

    #[test]
    fn big_hex() {
        let mut rng = rand::thread_rng();
        let v = (0..262_146).fold(String::new(), |mut acc, _| {
            let v = *b"0123456789abcdefABCDEF".choose(&mut rng).unwrap();
            acc.push(v.into());
            acc
        });

        // 128 * 1024 + 1
        let parsed: HexString = v.parse().unwrap();
        assert_eq!(parsed.to_lower(), v.to_lowercase());
        assert_eq!(parsed.len(), 131_073);
    }
}
