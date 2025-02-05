use std::{
    borrow::{Borrow, BorrowMut},
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    str::FromStr,
};

use crate::{utils, HexVectorError};

/// A hex string of variable length
///
/// For hex strings of constant length see [`HexArray`](crate::HexArray)
///
/// ## Example:
/// ```
/// use hex_str::HexVector;
///
/// // byte arrays are always valid
/// let a = HexVector::new([0x01, 0xde]);
/// assert_eq!(a, "01de");
/// assert_eq!(a, [0x01, 0xde]);
///
/// let b: HexVector = "01de".parse().unwrap();
/// assert_eq!(a, b);
/// ```
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HexVector(Vec<u8>);

impl HexVector {
    /// Create a new `HexVector`.
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexVector;
    ///
    /// let v = HexVector::new([0x1a, 0x2b, 0x3c, 0x4d]);
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
    /// use hex_str::HexVector;
    ///
    /// let v: HexVector = "1A2B3c4d".parse().unwrap();
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
    /// use hex_str::HexVector;
    ///
    /// let v: HexVector = "1A2B3c4d".parse().unwrap();
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
    /// This is the same as using [`HexVector::from_str`]/[`str::parse`] but
    /// accepts `impl AsRef<[u8]>`.
    ///
    /// # Errors
    /// - if `bytes.len() % 2 != 0`
    /// - if `bytes` contains characters other than `[0-9a-fA-F]`
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexVector;
    ///
    /// let v = HexVector::try_parse("1A2B3c4d");
    /// assert_eq!(v.unwrap(), "1a2b3c4d");
    /// ```
    pub fn try_parse(bytes: impl AsRef<[u8]>) -> Result<Self, HexVectorError> {
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
    /// use hex_str::{HexVector, HexVectorError};
    ///
    /// let v = HexVector::try_parse_lower("1a2b3c4d");
    /// assert_eq!(v.unwrap(), "1a2b3c4d");
    ///
    /// let v = HexVector::try_parse_lower("1A2B3C4D");
    /// assert_eq!(v.unwrap_err(), HexVectorError::InvalidByte { a: b'1', b: b'A', index: 0 });
    pub fn try_parse_lower(bytes: impl AsRef<[u8]>) -> Result<Self, HexVectorError> {
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
    /// use hex_str::{HexVector, HexVectorError};
    ///
    /// let v = HexVector::try_parse_upper("1A2B3C4D");
    /// assert_eq!(v.unwrap(), "1a2b3c4d");
    ///
    /// let v = HexVector::try_parse_upper("1a2b3c4d");
    /// assert_eq!(v.unwrap_err(), HexVectorError::InvalidByte { a: b'1', b: b'a', index: 0 });
    pub fn try_parse_upper(bytes: impl AsRef<[u8]>) -> Result<Self, HexVectorError> {
        try_parse(bytes, utils::parse_upper)
    }

    /// Return a mutable reference to the inner array.
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexVector;
    ///
    /// let mut v = HexVector::new([0x1a, 0x2b]);
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
) -> Result<HexVector, HexVectorError> {
    let bytes = bytes.as_ref();
    if bytes.len() % 2 != 0 {
        return Err(HexVectorError::InvalidLength {
            encountered: bytes.len(),
        });
    }

    let mut ret = Vec::with_capacity(bytes.len() / 2);
    let mut i = 0;
    let mut j = 1;
    for _ in 0..ret.capacity() {
        let msb = unsafe { *bytes.get_unchecked(i) };
        let lsb = unsafe { *bytes.get_unchecked(j) };
        conversion_fn(msb, lsb)
            .ok_or(HexVectorError::InvalidByte { msb, lsb, index: i })
            .map(|w| ret.push(w))?;

        // if len == usize::MAX, this will overflow after the last iteration
        // which is fine
        i = i.wrapping_add(2);
        j = j.wrapping_add(2);
    }

    Ok(HexVector::new(ret))
}

impl Display for HexVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.to_lower(), f)
    }
}

impl Debug for HexVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HexVector")
            .field("inner", &self.to_string())
            .finish()
    }
}

impl FromStr for HexVector {
    type Err = HexVectorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_parse(s)
    }
}

impl From<Vec<u8>> for HexVector {
    fn from(value: Vec<u8>) -> Self {
        Self::new(value)
    }
}

impl From<&[u8]> for HexVector {
    fn from(value: &[u8]) -> Self {
        Self::new(value)
    }
}

impl From<HexVector> for Vec<u8> {
    fn from(value: HexVector) -> Self {
        value.0
    }
}

impl TryFrom<&'_ str> for HexVector {
    type Error = HexVectorError;

    fn try_from(value: &'_ str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for HexVector {
    type Error = HexVectorError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<const N: usize> PartialEq<[u8; N]> for HexVector {
    fn eq(&self, other: &[u8; N]) -> bool {
        &*self.0 == other
    }
}

impl<const N: usize> PartialEq<&[u8; N]> for HexVector {
    fn eq(&self, other: &&[u8; N]) -> bool {
        &*self.0 == *other
    }
}

impl PartialEq<[u8]> for HexVector {
    fn eq(&self, other: &[u8]) -> bool {
        self.0 == other
    }
}

impl PartialEq<&[u8]> for HexVector {
    fn eq(&self, other: &&[u8]) -> bool {
        self.0 == *other
    }
}

impl PartialEq<str> for HexVector {
    #[allow(clippy::many_single_char_names)]
    fn eq(&self, other: &str) -> bool {
        if other.len() % 2 == 0 && other.len() / 2 == self.0.len() {
            let bytes = other.as_bytes();
            let mut i = 0;
            let mut j = 1;
            for x in &*self.0 {
                let msb = unsafe { *bytes.get_unchecked(i) };
                let lsb = unsafe { *bytes.get_unchecked(j) };

                let Some(converted) = utils::parse(msb, lsb) else {
                    return false;
                };

                if *x != converted {
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

impl PartialEq<&str> for HexVector {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<String> for HexVector {
    fn eq(&self, other: &String) -> bool {
        self == other.as_str()
    }
}

impl Deref for HexVector {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HexVector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<HexVector> for HexVector {
    fn as_ref(&self) -> &HexVector {
        self
    }
}

impl AsMut<HexVector> for HexVector {
    fn as_mut(&mut self) -> &mut HexVector {
        self
    }
}

impl AsRef<[u8]> for HexVector {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for HexVector {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl Borrow<Vec<u8>> for HexVector {
    fn borrow(&self) -> &Vec<u8> {
        self
    }
}

impl BorrowMut<Vec<u8>> for HexVector {
    fn borrow_mut(&mut self) -> &mut Vec<u8> {
        self
    }
}

impl Borrow<[u8]> for HexVector {
    fn borrow(&self) -> &[u8] {
        self
    }
}

impl BorrowMut<[u8]> for HexVector {
    fn borrow_mut(&mut self) -> &mut [u8] {
        self
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for HexVector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl serde::de::Visitor<'_> for Visitor {
            type Value = HexVector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("hex string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse().map_err(|err| E::custom(err))
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for HexVector {
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

    use super::HexVector;

    #[test]
    fn big_hex() {
        let mut rng = rand::thread_rng();
        let v = (0..262_146).fold(String::new(), |mut acc, _| {
            let v = *b"0123456789abcdefABCDEF".choose(&mut rng).unwrap();
            acc.push(v.into());
            acc
        });

        // 128 * 1024 + 1
        let parsed: HexVector = v.parse().unwrap();
        assert_eq!(parsed.to_lower(), v.to_lowercase());
        assert_eq!(parsed.len(), 131_073);
    }
}
