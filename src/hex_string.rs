use std::{
    borrow::{Borrow, BorrowMut},
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    str::FromStr,
};

use crate::{utils, Error};

/// A hex string of specific length
///
/// [`HexString`] of length `N`, where `N` denotes the length of its internal
/// array, not the length of its textual representation.
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
/// let b: HexString<2> = "01de".parse().unwrap();
/// assert_eq!(a, b);
/// ```
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HexString<const N: usize>([u8; N]);

impl<const N: usize> HexString<N> {
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
    pub fn new(v: impl Into<[u8; N]>) -> Self {
        Self(v.into())
    }

    /// Convert the `HexString` to its string representation, lowercase.
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexString;
    ///
    /// let v: HexString::<4> = "1A2B3c4d".parse().unwrap();
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

    /// Convert the `HexString` to its string representation, uppercase.
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexString;
    ///
    /// let v: HexString::<4> = "1A2B3c4d".parse().unwrap();
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

    /// Try to parse `value`, both lowercase and uppercase characters allowed.
    ///
    /// This is the same as using [`HexString::from_str`]/[`str::parse`] but
    /// accepts `impl AsRef<[u8]>`.
    ///
    /// # Errors
    /// - if `bytes.len() != 2*N`
    /// - if `bytes` contains characters other than `[0-9a-fA-F]`
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexString;
    ///
    /// let v = HexString::<4>::try_parse("1A2B3c4d");
    /// assert_eq!(v.unwrap(), "1a2b3c4d");
    /// ```
    pub fn try_parse(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        try_parse(bytes, utils::parse_quartet)
    }

    /// Try to parse `value`, only lowercase characters allowed.
    ///
    /// # Errors
    /// - if `bytes.len() != 2*N`
    /// - if `bytes` contains characters other than `[0-9a-f]`
    ///
    /// # Example:
    /// ```
    /// use hex_str::{HexString, Error};
    ///
    /// let v = HexString::<4>::try_parse_lower("1a2b3c4d");
    /// assert_eq!(v.unwrap(), "1a2b3c4d");
    ///
    /// let v = HexString::<4>::try_parse_lower("1A2B3C4D");
    /// assert_eq!(v.unwrap_err(), Error::InvalidCharacter { v: b'A', index: 1 });
    pub fn try_parse_lower(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        try_parse(bytes, utils::parse_quartet_lower)
    }

    /// Try to parse `value`, only uppercase characters allowed.
    ///
    /// # Errors
    /// - if `bytes.len() != 2*N`
    /// - if `bytes` contains characters other than `[0-9A-F]`
    ///
    /// # Example:
    /// ```
    /// use hex_str::{HexString, Error};
    ///
    /// let v = HexString::<4>::try_parse_upper("1A2B3C4D");
    /// assert_eq!(v.unwrap(), "1a2b3c4d");
    ///
    /// let v = HexString::<4>::try_parse_upper("1a2b3c4d");
    /// assert_eq!(v.unwrap_err(), Error::InvalidCharacter { v: b'a', index: 1 });
    pub fn try_parse_upper(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        try_parse(bytes, utils::parse_quartet_upper)
    }

    /// Return a reference to the inner array.
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexString;
    ///
    /// let v = HexString::new([0x1a, 0x2b]);
    ///
    /// assert_eq!(v.as_array(), &[0x1a, 0x2b]);
    /// ```
    #[must_use]
    pub fn as_array(&self) -> &[u8; N] {
        &self.0
    }

    /// Return a mutable reference to the inner array.
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexString;
    ///
    /// let mut v = HexString::new([0x1a, 0x2b]);
    /// let mut_array = v.as_mut_array();
    ///
    /// mut_array.iter_mut().for_each(|v| *v = 0);
    ///
    /// assert_eq!(v, "0000");
    /// ```
    #[must_use]
    pub fn as_mut_array(&mut self) -> &mut [u8; N] {
        &mut self.0
    }
}

fn try_parse<const N: usize>(
    bytes: impl AsRef<[u8]>,
    quartet_conversion_fn: impl Fn(u8) -> Option<u8>,
) -> Result<HexString<N>, Error> {
    let bytes = bytes.as_ref();
    if bytes.len() % 2 != 0 || bytes.len() / 2 != N {
        return Err(Error::InvalidLength {
            expected: N * 2,
            encountered: bytes.len(),
        });
    }

    let mut ret = [0; N];
    let mut i = 0;
    let mut j = 1;
    for v in &mut ret {
        let a = unsafe { *bytes.get_unchecked(i) };
        let a = quartet_conversion_fn(a).ok_or(Error::InvalidCharacter { v: a, index: i })?;

        let b = unsafe { *bytes.get_unchecked(j) };
        let b = quartet_conversion_fn(b).ok_or(Error::InvalidCharacter { v: b, index: j })?;

        *v = a << 4 | b;

        i = i.wrapping_add(2);
        j = j.wrapping_add(2);
    }

    Ok(HexString::new(ret))
}

impl<const N: usize> Display for HexString<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.to_lower(), f)
    }
}

impl<const N: usize> Debug for HexString<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HexString")
            .field("n", &N)
            .field("inner", &self.to_string())
            .finish()
    }
}

impl<const N: usize> FromStr for HexString<N> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_parse(s)
    }
}

impl<const N: usize> From<[u8; N]> for HexString<N> {
    fn from(value: [u8; N]) -> Self {
        Self::new(value)
    }
}

impl<const N: usize> From<HexString<N>> for [u8; N] {
    fn from(value: HexString<N>) -> Self {
        value.0
    }
}

impl<'a, const N: usize> TryFrom<&'a str> for HexString<N> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<const N: usize> TryFrom<String> for HexString<N> {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<const N: usize> PartialEq<[u8; N]> for HexString<N> {
    fn eq(&self, other: &[u8; N]) -> bool {
        &self.0 == other
    }
}

impl<const N: usize> PartialEq<&[u8; N]> for HexString<N> {
    fn eq(&self, other: &&[u8; N]) -> bool {
        &self.0 == *other
    }
}

impl<const N: usize> PartialEq<[u8]> for HexString<N> {
    fn eq(&self, other: &[u8]) -> bool {
        self.0 == other
    }
}

impl<const N: usize> PartialEq<&[u8]> for HexString<N> {
    fn eq(&self, other: &&[u8]) -> bool {
        self.0 == *other
    }
}

impl<const N: usize> PartialEq<str> for HexString<N> {
    fn eq(&self, other: &str) -> bool {
        if other.len() % 2 == 0 && other.len() / 2 == self.0.len() {
            let bytes = other.as_bytes();
            let mut i = 0;
            let mut j = 1;
            for v in &self.0 {
                let Some(a) = utils::parse_quartet(unsafe { *bytes.get_unchecked(i) }) else {
                    return false;
                };
                let Some(b) = utils::parse_quartet(unsafe { *bytes.get_unchecked(j) }) else {
                    return false;
                };

                if *v != a << 4 | b {
                    return false;
                }

                i = i.wrapping_add(2);
                j = j.wrapping_add(2);
            }

            true
        } else {
            false
        }
    }
}

impl<const N: usize> PartialEq<&str> for HexString<N> {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl<const N: usize> PartialEq<String> for HexString<N> {
    fn eq(&self, other: &String) -> bool {
        self == other.as_str()
    }
}

impl<const N: usize> Deref for HexString<N> {
    type Target = [u8; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for HexString<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> AsRef<HexString<N>> for HexString<N> {
    fn as_ref(&self) -> &HexString<N> {
        self
    }
}

impl<const N: usize> AsMut<HexString<N>> for HexString<N> {
    fn as_mut(&mut self) -> &mut HexString<N> {
        self
    }
}

impl<const N: usize> AsRef<[u8; N]> for HexString<N> {
    fn as_ref(&self) -> &[u8; N] {
        &self.0
    }
}

impl<const N: usize> AsMut<[u8; N]> for HexString<N> {
    fn as_mut(&mut self) -> &mut [u8; N] {
        &mut self.0
    }
}

impl<const N: usize> AsRef<[u8]> for HexString<N> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const N: usize> AsMut<[u8]> for HexString<N> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl<const N: usize> AsRef<HexString<N>> for [u8; N] {
    fn as_ref(&self) -> &HexString<N> {
        unsafe { &*(self as *const [u8; N]).cast() }
    }
}

impl<const N: usize> AsMut<HexString<N>> for [u8; N] {
    fn as_mut(&mut self) -> &mut HexString<N> {
        unsafe { &mut *(self as *mut [u8; N]).cast() }
    }
}

impl<const N: usize> Borrow<[u8; N]> for HexString<N> {
    fn borrow(&self) -> &[u8; N] {
        self
    }
}

impl<const N: usize> BorrowMut<[u8; N]> for HexString<N> {
    fn borrow_mut(&mut self) -> &mut [u8; N] {
        self
    }
}

impl<const N: usize> Borrow<[u8]> for HexString<N> {
    fn borrow(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<const N: usize> BorrowMut<[u8]> for HexString<N> {
    fn borrow_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl<const N: usize> Borrow<HexString<N>> for [u8; N] {
    fn borrow(&self) -> &HexString<N> {
        self.as_ref()
    }
}

impl<const N: usize> BorrowMut<HexString<N>> for [u8; N] {
    fn borrow_mut(&mut self) -> &mut HexString<N> {
        self.as_mut()
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize> serde::Deserialize<'de> for HexString<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor<const O: usize>;

        impl<'de, const O: usize> serde::de::Visitor<'de> for Visitor<O> {
            type Value = HexString<O>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_fmt(format_args!("hex string of length `{O}`"))
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
impl<const N: usize> serde::Serialize for HexString<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

#[cfg(feature = "rand")]
impl<const N: usize> rand::distributions::Distribution<HexString<N>>
    for rand::distributions::Standard
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> HexString<N> {
        HexString::new(rng.gen::<[u8; N]>())
    }
}
