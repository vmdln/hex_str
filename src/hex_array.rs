use core::{mem, ptr};
use std::{
    borrow::{Borrow, BorrowMut},
    fmt::{Debug, Display},
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use crate::{utils, HexArrayError, HexSlice, HexVector};

/// A hex string of constant length
///
/// A hex string of length `N`, where `N` denotes the length of its binary
/// representation, not the length of its textual representation.
/// For hex strings of variable length see [`HexVector`](crate::HexVector)
///
/// ## Example:
/// ```
/// use hex_str::HexArray;
///
/// // byte arrays are always valid
/// let a = HexArray::new([0x01, 0xde]);
/// assert_eq!(a, *"01de");
/// assert_eq!(a, [0x01, 0xde]);
///
/// let b: HexArray<2> = "01de".parse().unwrap();
/// assert_eq!(a, b);
/// ```
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HexArray<const N: usize>([u8; N]);

impl<const N: usize> HexArray<N> {
    /// Create a new `HexArray`.
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexArray;
    ///
    /// let v = HexArray::new([0x1a, 0x2b, 0x3c, 0x4d]);
    /// assert_eq!(v, [0x1a, 0x2b, 0x3c, 0x4d]);
    /// assert_eq!(v, *"1a2b3c4d");
    /// ```
    #[must_use]
    pub fn new(v: impl Into<[u8; N]>) -> Self {
        Self(v.into())
    }

    /// Create a new `HexArray` directly on the heap.
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexArray;
    ///
    /// let v = HexArray::new_boxed([0x1a, 0x2b, 0x3c, 0x4d]);
    /// assert_eq!(*v, [0x1a, 0x2b, 0x3c, 0x4d]);
    /// assert_eq!(*v, *"1a2b3c4d");
    /// ```
    #[must_use]
    pub fn new_boxed(v: impl Into<Box<[u8; N]>>) -> Box<Self> {
        unsafe { mem::transmute(v.into()) }
    }

    /// Try to parse `bytes`, both lowercase and uppercase characters allowed.
    ///
    /// This is the same as using [`HexArray::from_str`]/[`str::parse`] but
    /// accepts `impl AsRef<[u8]>`.
    ///
    /// # Errors
    /// - if `bytes.len() != 2*N`
    /// - if `bytes` contains characters other than `[0-9a-fA-F]`
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexArray;
    ///
    /// let v = HexArray::<4>::try_parse("1A2B3c4d");
    /// assert_eq!(v.unwrap(), *"1a2b3c4d");
    /// ```
    pub fn try_parse(bytes: impl AsRef<[u8]>) -> Result<Self, HexArrayError> {
        try_parse(bytes, utils::parse)
    }

    /// Try to parse `bytes`, both lowercase and uppercase characters allowed,
    /// directly on the heap.
    ///
    /// This is the same as using [`HexArray::from_str`]/[`str::parse`] but
    /// accepts `impl AsRef<[u8]>`.
    ///
    /// # Errors
    /// - if `bytes.len() != 2*N`
    /// - if `bytes` contains characters other than `[0-9a-fA-F]`
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexArray;
    ///
    /// let v = HexArray::<4>::try_parse_boxed("1A2B3c4d");
    /// assert_eq!(*v.unwrap(), *"1a2b3c4d");
    /// ```
    pub fn try_parse_boxed(bytes: impl AsRef<[u8]>) -> Result<Box<Self>, HexArrayError> {
        try_parse_boxed(bytes, utils::parse)
    }

    /// Try to parse `bytes`, only lowercase characters allowed.
    ///
    /// # Errors
    /// - if `bytes.len() != 2*N`
    /// - if `bytes` contains characters other than `[0-9a-f]`
    ///
    /// # Example:
    /// ```
    /// use hex_str::{HexArray, HexArrayError};
    ///
    /// let v = HexArray::<4>::try_parse_lower("1a2b3c4d");
    /// assert_eq!(v.unwrap(), *"1a2b3c4d");
    ///
    /// let v = HexArray::<4>::try_parse_lower("1A2B3C4D");
    /// assert_eq!(v.unwrap_err(), HexArrayError::InvalidByte { msb: b'1', lsb: b'A', index: 0 });
    pub fn try_parse_lower(bytes: impl AsRef<[u8]>) -> Result<Self, HexArrayError> {
        try_parse(bytes, utils::parse_lower)
    }

    /// Try to parse `bytes`, only lowercase characters allowed, directly on the
    /// heap.
    ///
    /// # Errors
    /// - if `bytes.len() != 2*N`
    /// - if `bytes` contains characters other than `[0-9a-f]`
    ///
    /// # Example:
    /// ```
    /// use hex_str::{HexArray, HexArrayError};
    ///
    /// let v = HexArray::<4>::try_parse_lower_boxed("1a2b3c4d");
    /// assert_eq!(*v.unwrap(), *"1a2b3c4d");
    ///
    /// let v = HexArray::<4>::try_parse_lower_boxed("1A2B3C4D");
    /// assert_eq!(v.unwrap_err(), HexArrayError::InvalidByte { msb: b'1', lsb: b'A', index: 0 });
    pub fn try_parse_lower_boxed(bytes: impl AsRef<[u8]>) -> Result<Box<Self>, HexArrayError> {
        try_parse_boxed(bytes, utils::parse_lower)
    }

    /// Try to parse `bytes`, only uppercase characters allowed.
    ///
    /// # Errors
    /// - if `bytes.len() != 2*N`
    /// - if `bytes` contains characters other than `[0-9A-F]`
    ///
    /// # Example:
    /// ```
    /// use hex_str::{HexArray, HexArrayError};
    ///
    /// let v = HexArray::<4>::try_parse_upper("1A2B3C4D");
    /// assert_eq!(v.unwrap(), *"1a2b3c4d");
    ///
    /// let v = HexArray::<4>::try_parse_upper("1a2b3c4d");
    /// assert_eq!(v.unwrap_err(), HexArrayError::InvalidByte { msb: b'1', lsb: b'a', index: 0 });
    pub fn try_parse_upper(bytes: impl AsRef<[u8]>) -> Result<Self, HexArrayError> {
        try_parse(bytes, utils::parse_upper)
    }

    /// Try to parse `bytes`, only uppercase characters allowed, directly on the
    /// heap.
    ///
    /// # Errors
    /// - if `bytes.len() != 2*N`
    /// - if `bytes` contains characters other than `[0-9A-F]`
    ///
    /// # Example:
    /// ```
    /// use hex_str::{HexArray, HexArrayError};
    ///
    /// let v = HexArray::<4>::try_parse_upper_boxed("1A2B3C4D");
    /// assert_eq!(*v.unwrap(), *"1a2b3c4d");
    ///
    /// let v = HexArray::<4>::try_parse_upper_boxed("1a2b3c4d");
    /// assert_eq!(v.unwrap_err(), HexArrayError::InvalidByte { msb: b'1', lsb: b'a', index: 0 });
    pub fn try_parse_upper_boxed(bytes: impl AsRef<[u8]>) -> Result<Box<Self>, HexArrayError> {
        try_parse_boxed(bytes, utils::parse_upper)
    }

    /// Return a reference to the inner array.
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexArray;
    ///
    /// let v = HexArray::new([0x1a, 0x2b]);
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
    /// use hex_str::HexArray;
    ///
    /// let mut v = HexArray::new([0x1a, 0x2b]);
    /// let mut_array = v.as_mut_array();
    ///
    /// mut_array.iter_mut().for_each(|v| *v = 0);
    ///
    /// assert_eq!(v, *"0000");
    /// ```
    #[must_use]
    pub fn as_mut_array(&mut self) -> &mut [u8; N] {
        &mut self.0
    }

    #[must_use]
    pub fn as_hex_slice(&self) -> &HexSlice {
        // Safety: `HexSlice` is `#[repr(transparent)]` `[u8]`
        unsafe { &*(ptr::from_ref(self.0.as_slice()) as *const HexSlice) }
    }

    #[must_use]
    pub fn as_mut_hex_slice(&mut self) -> &mut HexSlice {
        // Safety: `HexSlice` is `#[repr(transparent)]` `[u8]`
        unsafe { &mut *(ptr::from_mut(self.0.as_mut_slice()) as *mut HexSlice) }
    }
}

fn try_parse<const N: usize>(
    bytes: impl AsRef<[u8]>,
    conversion_fn: impl Fn(u8, u8) -> Option<u8>,
) -> Result<HexArray<N>, HexArrayError> {
    let bytes_ref = bytes.as_ref();
    if bytes_ref.len() % 2 != 0 || bytes_ref.len() / 2 != N {
        return Err(HexArrayError::InvalidLength {
            expected: N * 2,
            encountered: bytes_ref.len(),
        });
    }

    let mut uninitialized = [MaybeUninit::<u8>::uninit(); N];
    let mut i = 0;
    let mut j = 1;
    for v in &mut uninitialized {
        let msb = unsafe { *bytes_ref.get_unchecked(i) };
        let lsb = unsafe { *bytes_ref.get_unchecked(j) };
        conversion_fn(msb, lsb)
            .ok_or(HexArrayError::InvalidByte { msb, lsb, index: i })
            .map(|w| v.write(w))?;

        // if len == usize::MAX, this will overflow after the last iteration
        // which is fine
        i = i.wrapping_add(2);
        j = j.wrapping_add(2);
    }

    // we can't use `core::mem::transmute` here due to
    // https://github.com/rust-lang/rust/issues/61956
    let initialized = unsafe { uninitialized.as_ptr().cast::<[u8; N]>().read() };
    Ok(HexArray::new(initialized))
}

fn try_parse_boxed<const N: usize>(
    bytes: impl AsRef<[u8]>,
    conversion_fn: impl Fn(u8, u8) -> Option<u8>,
) -> Result<Box<HexArray<N>>, HexArrayError> {
    let bytes_ref = bytes.as_ref();
    if bytes_ref.len() % 2 != 0 || bytes_ref.len() / 2 != N {
        return Err(HexArrayError::InvalidLength {
            expected: N * 2,
            encountered: bytes_ref.len(),
        });
    }

    let mut uninitialized: Box<[MaybeUninit<u8>; N]> = unsafe { Box::new_uninit().assume_init() };
    // for (n, (v, (msb, lsb))) in ret
    //     .iter_mut()
    //     .zip(bytes.iter().copied().tuples())
    //     .enumerate()
    // {
    //     let converted =
    //         conversion_fn(msb, lsb).ok_or(HexArrayError::InvalidByte { msb, lsb, index: n })?;
    //     v.write(converted);
    // }
    let mut i = 0;
    let mut j = 1;
    for v in &mut *uninitialized {
        let msb = unsafe { *bytes_ref.get_unchecked(i) };
        let lsb = unsafe { *bytes_ref.get_unchecked(j) };
        conversion_fn(msb, lsb)
            .ok_or(HexArrayError::InvalidByte { msb, lsb, index: i })
            .map(|w| v.write(w))?;

        // if len == usize::MAX, this will overflow after the last iteration
        // which is fine
        i = i.wrapping_add(2);
        j = j.wrapping_add(2);
    }

    let initialized: Box<[u8; N]> = unsafe { mem::transmute(uninitialized) };
    Ok(HexArray::new_boxed(initialized))
}

impl<const N: usize> Display for HexArray<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.to_lower(), f)
    }
}

impl<const N: usize> Debug for HexArray<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HexArray")
            .field("n", &N)
            .field("inner", &self.to_string())
            .finish()
    }
}

impl<const N: usize> FromStr for HexArray<N> {
    type Err = HexArrayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_parse(s)
    }
}

impl<const N: usize> From<[u8; N]> for HexArray<N> {
    fn from(value: [u8; N]) -> Self {
        Self::new(value)
    }
}

impl<const N: usize> From<Box<[u8; N]>> for Box<HexArray<N>> {
    fn from(value: Box<[u8; N]>) -> Self {
        HexArray::new_boxed(value)
    }
}

impl<const N: usize> From<Box<HexArray<N>>> for Box<[u8; N]> {
    fn from(value: Box<HexArray<N>>) -> Self {
        // Safety: `HexArray` is `#[repr(transparent)]` `[u8; N]`
        unsafe { mem::transmute(value) }
    }
}

impl<'a, const N: usize> TryFrom<&'a str> for HexArray<N> {
    type Error = HexArrayError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<const N: usize> TryFrom<String> for HexArray<N> {
    type Error = HexArrayError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

// PartialEq
impl<const N: usize> PartialEq<HexSlice> for HexArray<N> {
    fn eq(&self, other: &HexSlice) -> bool {
        self == other.as_slice()
    }
}

impl<const N: usize> PartialEq<HexVector> for HexArray<N> {
    fn eq(&self, other: &HexVector) -> bool {
        self == other.as_hex_slice()
    }
}

impl<const N: usize> PartialEq<[u8; N]> for HexArray<N> {
    fn eq(&self, other: &[u8; N]) -> bool {
        self.0 == *other
    }
}

impl<const N: usize> PartialEq<[u8]> for HexArray<N> {
    fn eq(&self, other: &[u8]) -> bool {
        self.0 == other
    }
}

impl<const N: usize> PartialEq<str> for HexArray<N> {
    #[allow(clippy::many_single_char_names)]
    fn eq(&self, other: &str) -> bool {
        if other.len() % 2 == 0 && other.len() / 2 == self.0.len() {
            let bytes = other.as_bytes();
            let mut i = 0;
            let mut j = 1;
            for x in &self.0 {
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

impl<const N: usize> PartialEq<String> for HexArray<N> {
    fn eq(&self, other: &String) -> bool {
        self == other.as_str()
    }
}

impl<const N: usize> Deref for HexArray<N> {
    type Target = HexSlice;

    fn deref(&self) -> &Self::Target {
        self.as_hex_slice()
    }
}

impl<const N: usize> DerefMut for HexArray<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_hex_slice()
    }
}

// AsRef/Asmut
impl<const N: usize> AsRef<HexArray<N>> for HexArray<N> {
    fn as_ref(&self) -> &HexArray<N> {
        self
    }
}

impl<const N: usize> AsMut<HexArray<N>> for HexArray<N> {
    fn as_mut(&mut self) -> &mut HexArray<N> {
        self
    }
}

// HexArray -> HexSlice
impl<const N: usize> AsRef<HexSlice> for HexArray<N> {
    fn as_ref(&self) -> &HexSlice {
        self.as_hex_slice()
    }
}

impl<const N: usize> AsMut<HexSlice> for HexArray<N> {
    fn as_mut(&mut self) -> &mut HexSlice {
        self.as_mut_hex_slice()
    }
}

// HexArray -> [u8; N]
impl<const N: usize> AsRef<[u8; N]> for HexArray<N> {
    fn as_ref(&self) -> &[u8; N] {
        &self.0
    }
}

impl<const N: usize> AsMut<[u8; N]> for HexArray<N> {
    fn as_mut(&mut self) -> &mut [u8; N] {
        &mut self.0
    }
}

// HexArray -> [u8]
impl<const N: usize> AsRef<[u8]> for HexArray<N> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const N: usize> AsMut<[u8]> for HexArray<N> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

// [u8; N] -> HexArray
impl<const N: usize> AsRef<HexArray<N>> for [u8; N] {
    fn as_ref(&self) -> &HexArray<N> {
        // Safety: `HexArray` is `#[repr(transparent)]` `[u8; N]`
        unsafe { &*ptr::from_ref(self).cast() }
    }
}

impl<const N: usize> AsMut<HexArray<N>> for [u8; N] {
    fn as_mut(&mut self) -> &mut HexArray<N> {
        // Safety: `HexArray` is `#[repr(transparent)]` `[u8; N]`
        unsafe { &mut *ptr::from_mut(self).cast() }
    }
}

// Borrow
impl<const N: usize> Borrow<HexSlice> for HexArray<N> {
    fn borrow(&self) -> &HexSlice {
        self.as_hex_slice()
    }
}

impl<const N: usize> BorrowMut<HexSlice> for HexArray<N> {
    fn borrow_mut(&mut self) -> &mut HexSlice {
        self.as_mut_hex_slice()
    }
}

impl<const N: usize> Borrow<[u8; N]> for HexArray<N> {
    fn borrow(&self) -> &[u8; N] {
        &self.0
    }
}

impl<const N: usize> BorrowMut<[u8; N]> for HexArray<N> {
    fn borrow_mut(&mut self) -> &mut [u8; N] {
        &mut self.0
    }
}

impl<const N: usize> Borrow<[u8]> for HexArray<N> {
    fn borrow(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl<const N: usize> BorrowMut<[u8]> for HexArray<N> {
    fn borrow_mut(&mut self) -> &mut [u8] {
        self.0.as_mut_slice()
    }
}

impl<const N: usize> Borrow<HexArray<N>> for Box<[u8; N]> {
    fn borrow(&self) -> &HexArray<N> {
        // Safety: HexArray is #[repr(transparent)]
        unsafe { &*ptr::from_ref(self).cast() }
    }
}

impl<const N: usize> BorrowMut<HexArray<N>> for Box<[u8; N]> {
    fn borrow_mut(&mut self) -> &mut HexArray<N> {
        // Safety: HexArray is #[repr(transparent)]
        unsafe { &mut *ptr::from_mut(self).cast() }
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize> serde::Deserialize<'de> for HexArray<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor<const O: usize>;

        impl<const O: usize> serde::de::Visitor<'_> for Visitor<O> {
            type Value = HexArray<O>;

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
impl<const N: usize> serde::Serialize for HexArray<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

#[cfg(feature = "rand")]
impl<const N: usize> rand::distributions::Distribution<HexArray<N>>
    for rand::distributions::Standard
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> HexArray<N> {
        let mut uninitialized = [MaybeUninit::<u8>::uninit(); N];
        for v in &mut uninitialized {
            v.write(rng.gen());
        }

        // we can't use `core::mem::transmute` here due to
        // https://github.com/rust-lang/rust/issues/61956
        let initialized = unsafe { uninitialized.as_ptr().cast::<[u8; N]>().read() };
        HexArray::new(initialized)
    }
}

#[cfg(feature = "rand")]
impl<const N: usize> rand::distributions::Distribution<Box<HexArray<N>>>
    for rand::distributions::Standard
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Box<HexArray<N>> {
        let mut uninitialized: Box<[MaybeUninit<u8>; N]> =
            unsafe { Box::new_uninit().assume_init() };
        for v in &mut *uninitialized {
            v.write(rng.gen());
        }

        let initialized: Box<[u8; N]> = unsafe { mem::transmute(uninitialized) };
        HexArray::new_boxed(initialized)
    }
}

#[cfg(all(test, feature = "rand"))]
mod tests {
    use rand::seq::SliceRandom;

    use super::HexArray;

    #[test]
    fn rand_stack() {
        // 32 * 1024 - 1
        let _: HexArray<32_767> = rand::random();
    }

    #[test]
    fn rand_heap() {
        // 1024 * 1024 -1
        let _: Box<HexArray<1_048_575>> = rand::random();
    }

    #[test]
    fn big_hex() {
        let mut rng = rand::thread_rng();
        let v = (0..1_048_576).fold(String::new(), |mut acc, _| {
            let v = *b"0123456789abcdefABCDEF".choose(&mut rng).unwrap();
            acc.push(v.into());
            acc
        });

        // 1024 * 1024 -1
        let parsed: Box<HexArray<524_288>> = HexArray::try_parse_boxed(&v).unwrap();
        assert_eq!(parsed.to_lower(), v.to_lowercase());
    }
}
