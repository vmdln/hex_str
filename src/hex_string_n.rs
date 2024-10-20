use std::{
    borrow::{Borrow, BorrowMut},
    fmt::{Debug, Display},
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    ptr,
    str::FromStr,
};

use crate::{utils, HexStringNError};

/// A hex string of specific length
///
/// A hex string of length `N`, where `N` denotes the length of its internal
/// array, not the length of its textual representation.
/// For hex strings of variable length see [`HexString`](crate::HexString)
///
/// ## Example:
/// ```
/// use hex_str::HexStringN;
///
/// // byte arrays are always valid
/// let a = HexStringN::new([0x01, 0xde]);
/// assert_eq!(a, "01de");
/// assert_eq!(a, [0x01, 0xde]);
///
/// let b: HexStringN<2> = "01de".parse().unwrap();
/// assert_eq!(a, b);
/// ```
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HexStringN<const N: usize>(Box<[u8; N]>);

impl<const N: usize> HexStringN<N> {
    /// Create a new `HexStringN`.
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexStringN;
    ///
    /// let v = HexStringN::new([0x1a, 0x2b, 0x3c, 0x4d]);
    /// assert_eq!(v, [0x1a, 0x2b, 0x3c, 0x4d]);
    /// assert_eq!(v, "1a2b3c4d");
    /// ```
    #[must_use]
    pub fn new(v: impl Into<Box<[u8; N]>>) -> Self {
        Self(v.into())
    }

    /// Convert `self` to its string representation, lowercase.
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexStringN;
    ///
    /// let v: HexStringN::<4> = "1A2B3c4d".parse().unwrap();
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
    /// use hex_str::HexStringN;
    ///
    /// let v: HexStringN::<4> = "1A2B3c4d".parse().unwrap();
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
    /// This is the same as using [`HexStringN::from_str`]/[`str::parse`] but
    /// accepts `impl AsRef<[u8]>`.
    ///
    /// # Errors
    /// - if `bytes.len() != 2*N`
    /// - if `bytes` contains characters other than `[0-9a-fA-F]`
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexStringN;
    ///
    /// let v = HexStringN::<4>::try_parse("1A2B3c4d");
    /// assert_eq!(v.unwrap(), "1a2b3c4d");
    /// ```
    pub fn try_parse(bytes: impl AsRef<[u8]>) -> Result<Self, HexStringNError> {
        try_parse(bytes, utils::parse)
    }

    /// Try to parse `bytes`, only lowercase characters allowed.
    ///
    /// # Errors
    /// - if `bytes.len() != 2*N`
    /// - if `bytes` contains characters other than `[0-9a-f]`
    ///
    /// # Example:
    /// ```
    /// use hex_str::{HexStringN, HexStringNError};
    ///
    /// let v = HexStringN::<4>::try_parse_lower("1a2b3c4d");
    /// assert_eq!(v.unwrap(), "1a2b3c4d");
    ///
    /// let v = HexStringN::<4>::try_parse_lower("1A2B3C4D");
    /// assert_eq!(v.unwrap_err(), HexStringNError::InvalidByte { a: b'1', b: b'A', index: 0 });
    pub fn try_parse_lower(bytes: impl AsRef<[u8]>) -> Result<Self, HexStringNError> {
        try_parse(bytes, utils::parse_lower)
    }

    /// Try to parse `bytes`, only uppercase characters allowed.
    ///
    /// # Errors
    /// - if `bytes.len() != 2*N`
    /// - if `bytes` contains characters other than `[0-9A-F]`
    ///
    /// # Example:
    /// ```
    /// use hex_str::{HexStringN, HexStringNError};
    ///
    /// let v = HexStringN::<4>::try_parse_upper("1A2B3C4D");
    /// assert_eq!(v.unwrap(), "1a2b3c4d");
    ///
    /// let v = HexStringN::<4>::try_parse_upper("1a2b3c4d");
    /// assert_eq!(v.unwrap_err(), HexStringNError::InvalidByte { a: b'1', b: b'a', index: 0 });
    pub fn try_parse_upper(bytes: impl AsRef<[u8]>) -> Result<Self, HexStringNError> {
        try_parse(bytes, utils::parse_upper)
    }

    /// Return a reference to the inner array.
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexStringN;
    ///
    /// let v = HexStringN::new([0x1a, 0x2b]);
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
    /// use hex_str::HexStringN;
    ///
    /// let mut v = HexStringN::new([0x1a, 0x2b]);
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
    conversion_fn: impl Fn(u8, u8) -> Option<u8>,
) -> Result<HexStringN<N>, HexStringNError> {
    let bytes = bytes.as_ref();
    if bytes.len() % 2 != 0 || bytes.len() / 2 != N {
        return Err(HexStringNError::InvalidLength {
            expected: N * 2,
            encountered: bytes.len(),
        });
    }

    let mut ret: Box<[MaybeUninit<u8>; N]> = unsafe { Box::new_uninit().assume_init() };
    let mut i = 0;
    let mut j = 1;
    for v in &mut *ret {
        let a = unsafe { *bytes.get_unchecked(i) };
        let b = unsafe { *bytes.get_unchecked(j) };
        conversion_fn(a, b)
            .ok_or(HexStringNError::InvalidByte { a, b, index: i })
            .map(|w| v.write(w))?;

        // if len == usize::MAX, this will overflow after the last iteration
        // which is fine
        i = i.wrapping_add(2);
        j = j.wrapping_add(2);
    }

    let ret: Box<[u8; N]> = unsafe { std::mem::transmute(ret) };
    Ok(HexStringN::new(ret))
}

impl<const N: usize> Display for HexStringN<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.to_lower(), f)
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

impl<const N: usize> FromStr for HexStringN<N> {
    type Err = HexStringNError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_parse(s)
    }
}

impl<const N: usize> From<[u8; N]> for HexStringN<N> {
    fn from(value: [u8; N]) -> Self {
        Self::new(value)
    }
}

impl<const N: usize> From<Box<[u8; N]>> for HexStringN<N> {
    fn from(value: Box<[u8; N]>) -> Self {
        Self::new(value)
    }
}

impl<const N: usize> From<HexStringN<N>> for Box<[u8; N]> {
    fn from(value: HexStringN<N>) -> Self {
        value.0
    }
}

impl<'a, const N: usize> TryFrom<&'a str> for HexStringN<N> {
    type Error = HexStringNError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<const N: usize> TryFrom<String> for HexStringN<N> {
    type Error = HexStringNError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<const N: usize> PartialEq<[u8; N]> for HexStringN<N> {
    fn eq(&self, other: &[u8; N]) -> bool {
        &*self.0 == other
    }
}

impl<const N: usize> PartialEq<&[u8; N]> for HexStringN<N> {
    fn eq(&self, other: &&[u8; N]) -> bool {
        &*self.0 == *other
    }
}

impl<const N: usize> PartialEq<[u8]> for HexStringN<N> {
    fn eq(&self, other: &[u8]) -> bool {
        *self.0 == other
    }
}

impl<const N: usize> PartialEq<&[u8]> for HexStringN<N> {
    fn eq(&self, other: &&[u8]) -> bool {
        *self.0 == *other
    }
}

impl<const N: usize> PartialEq<str> for HexStringN<N> {
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

impl<const N: usize> PartialEq<&str> for HexStringN<N> {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl<const N: usize> PartialEq<String> for HexStringN<N> {
    fn eq(&self, other: &String) -> bool {
        self == other.as_str()
    }
}

impl<const N: usize> Deref for HexStringN<N> {
    type Target = [u8; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for HexStringN<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> AsRef<HexStringN<N>> for HexStringN<N> {
    fn as_ref(&self) -> &HexStringN<N> {
        self
    }
}

impl<const N: usize> AsMut<HexStringN<N>> for HexStringN<N> {
    fn as_mut(&mut self) -> &mut HexStringN<N> {
        self
    }
}

impl<const N: usize> AsRef<[u8; N]> for HexStringN<N> {
    fn as_ref(&self) -> &[u8; N] {
        &self.0
    }
}

impl<const N: usize> AsMut<[u8; N]> for HexStringN<N> {
    fn as_mut(&mut self) -> &mut [u8; N] {
        &mut self.0
    }
}

impl<const N: usize> AsRef<[u8]> for HexStringN<N> {
    fn as_ref(&self) -> &[u8] {
        &*self.0
    }
}

impl<const N: usize> AsMut<[u8]> for HexStringN<N> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut *self.0
    }
}

impl<const N: usize> Borrow<[u8; N]> for HexStringN<N> {
    fn borrow(&self) -> &[u8; N] {
        self
    }
}

impl<const N: usize> BorrowMut<[u8; N]> for HexStringN<N> {
    fn borrow_mut(&mut self) -> &mut [u8; N] {
        self
    }
}

impl<const N: usize> Borrow<[u8]> for HexStringN<N> {
    fn borrow(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<const N: usize> BorrowMut<[u8]> for HexStringN<N> {
    fn borrow_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl<const N: usize> Borrow<HexStringN<N>> for Box<[u8; N]> {
    fn borrow(&self) -> &HexStringN<N> {
        // Safety: HexString is #[repr(transparent)]
        unsafe { &*ptr::from_ref(self).cast() }
    }
}

impl<const N: usize> BorrowMut<HexStringN<N>> for Box<[u8; N]> {
    fn borrow_mut(&mut self) -> &mut HexStringN<N> {
        // Safety: HexString is #[repr(transparent)]
        unsafe { &mut *ptr::from_mut(self).cast() }
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize> serde::Deserialize<'de> for HexStringN<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor<const O: usize>;

        impl<'de, const O: usize> serde::de::Visitor<'de> for Visitor<O> {
            type Value = HexStringN<O>;

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
impl<const N: usize> serde::Serialize for HexStringN<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

#[cfg(feature = "rand")]
impl<const N: usize> rand::distributions::Distribution<HexStringN<N>>
    for rand::distributions::Standard
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> HexStringN<N> {
        let mut ret: Box<[MaybeUninit<u8>; N]> = unsafe { Box::new_uninit().assume_init() };
        for v in &mut *ret {
            v.write(rng.gen());
        }

        let ret: Box<[u8; N]> = unsafe { std::mem::transmute(ret) };
        HexStringN::new(ret)
    }
}

#[cfg(all(test, feature = "rand"))]
mod tests {
    use rand::seq::SliceRandom;

    use super::HexStringN;

    #[test]
    fn big_rand() {
        // 128 * 1024 + 1
        let _: HexStringN<131_073> = rand::random();
    }

    #[test]
    fn big_hex() {
        let mut rng = rand::thread_rng();
        let v = (0..262_146).fold(String::new(), |mut acc, _| {
            let v = *b"0123456789abcdefABCDEF".choose(&mut rng).unwrap();
            acc.push(v.into());
            acc
        });

        // 128 * 1024 + 1
        let parsed: HexStringN<131_073> = v.parse().unwrap();
        assert_eq!(parsed.to_lower(), v.to_lowercase());
    }
}
