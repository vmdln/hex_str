use core::{
    borrow::{Borrow, BorrowMut},
    ptr,
};
use std::ops::{Deref, DerefMut};

use crate::{utils, HexArray, HexVector};

extern crate alloc;

#[derive(PartialEq, Eq)]
#[repr(transparent)]
pub struct HexSlice([u8]);

impl HexSlice {
    /// Coerce reference into a `HexSlice`.
    ///
    /// # Example
    /// ```
    /// use hex_str::HexSlice;
    ///
    /// let _ = HexSlice::new(&[0x1a, 0x2b, 0x3c, 0x4d]);
    /// ```
    #[must_use]
    pub fn new<T>(v: &T) -> &Self
    where
        T: AsRef<Self> + ?Sized,
    {
        v.as_ref()
    }

    /// Coerce mutable reference into a `HexSlice`.
    ///
    /// # Example
    /// ```
    /// use hex_str::HexSlice;
    ///
    /// let _ = HexSlice::new_mut(&mut [0x1a, 0x2b, 0x3c, 0x4d]);
    /// ```
    #[must_use]
    pub fn new_mut<T>(v: &mut T) -> &mut Self
    where
        T: AsMut<Self> + ?Sized,
    {
        v.as_mut()
    }

    /// Return a reference to the inner slice.
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexSlice;
    ///
    /// let v = HexSlice::new(&[0x1a, 0x2b]);
    ///
    /// assert_eq!(v.as_slice(), &[0x1a, 0x2b]);
    /// ```
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    /// Return a mutable reference to the inner slice.
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexSlice;
    ///
    /// let mut arr = [0x1a, 0x2b];
    /// let v = HexSlice::new_mut(&mut arr);
    ///
    /// let inner = v.as_mut_slice();
    /// assert_eq!(inner, &[0x1a, 0x2b]);
    ///
    /// inner.iter_mut().for_each(|v| *v = 0);
    /// assert_eq!(v, &[0, 0]);
    /// ```
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.0
    }

    /// Convert `self` to its string representation, lowercase.
    ///
    /// # Example:
    /// ```
    /// use hex_str::HexSlice;
    ///
    /// let v = HexSlice::new(&[0x1a, 0x2b, 0x3c, 0x4d]);
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
    /// use hex_str::HexSlice;
    ///
    /// let v = HexSlice::new(&[0x1a, 0x2b, 0x3c, 0x4d]);
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
}

// Debug
impl alloc::fmt::Debug for HexSlice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("HexSlice").field(&self.to_lower()).finish()
    }
}

// PartialEq
impl<const N: usize> PartialEq<HexArray<N>> for HexSlice {
    fn eq(&self, other: &HexArray<N>) -> bool {
        self == other.as_hex_slice()
    }
}

impl PartialEq<HexVector> for HexSlice {
    fn eq(&self, other: &HexVector) -> bool {
        self == other.as_hex_slice()
    }
}

impl PartialEq<[u8]> for HexSlice {
    fn eq(&self, other: &[u8]) -> bool {
        self.0 == *other
    }
}

impl<const N: usize> PartialEq<[u8; N]> for HexSlice {
    fn eq(&self, other: &[u8; N]) -> bool {
        self.0 == *other
    }
}

impl PartialEq<str> for HexSlice {
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

// AsRef/Asmut
impl AsRef<HexSlice> for HexSlice {
    fn as_ref(&self) -> &HexSlice {
        self
    }
}

impl AsMut<HexSlice> for HexSlice {
    fn as_mut(&mut self) -> &mut HexSlice {
        self
    }
}

// [u8] -> HexSlice
impl AsRef<HexSlice> for [u8] {
    fn as_ref(&self) -> &HexSlice {
        // Safety: `HexSlice` is `#[repr(transparent)]` `[u8]`
        unsafe { &*(ptr::from_ref(self) as *const HexSlice) }
    }
}

impl AsMut<HexSlice> for [u8] {
    fn as_mut(&mut self) -> &mut HexSlice {
        // Safety: `HexSlice` is `#[repr(transparent)]` `[u8]`
        unsafe { &mut *(ptr::from_mut(self) as *mut HexSlice) }
    }
}

// [u8; N] -> HexSlice
impl<const N: usize> AsRef<HexSlice> for [u8; N] {
    fn as_ref(&self) -> &HexSlice {
        // Safety: `HexSlice` is `#[repr(transparent)]` `[u8]`
        unsafe { &*(ptr::from_ref(self.as_slice()) as *const HexSlice) }
    }
}

impl<const N: usize> AsMut<HexSlice> for [u8; N] {
    fn as_mut(&mut self) -> &mut HexSlice {
        // Safety: `HexSlice` is `#[repr(transparent)]` `[u8]`
        unsafe { &mut *(ptr::from_mut(self.as_mut_slice()) as *mut HexSlice) }
    }
}

// HexSlice -> [u8]
impl AsRef<[u8]> for HexSlice {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for HexSlice {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

// Deref
impl Deref for HexSlice {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HexSlice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Borrow
impl Borrow<[u8]> for HexSlice {
    fn borrow(&self) -> &[u8] {
        &self.0
    }
}

impl BorrowMut<[u8]> for HexSlice {
    fn borrow_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl Borrow<HexSlice> for [u8] {
    fn borrow(&self) -> &HexSlice {
        HexSlice::new(self)
    }
}

impl BorrowMut<HexSlice> for [u8] {
    fn borrow_mut(&mut self) -> &mut HexSlice {
        HexSlice::new_mut(self)
    }
}

#[cfg(test)]
mod tests {
    use super::HexSlice;
    // new
    #[test]
    fn new_sized() {
        let _ = HexSlice::new(&[1, 2, 3]);
    }

    #[test]
    fn new_unsized() {
        let _ = HexSlice::new([1, 2, 3].as_slice());
    }

    #[test]
    fn new_mut_sized() {
        let _ = HexSlice::new_mut(&mut [1, 2, 3]);
    }

    #[test]
    fn new_mut_unsized() {
        let _ = HexSlice::new_mut([1, 2, 3].as_mut_slice());
    }

    // PartialEq
    #[test]
    fn eq_slice() {
        let a = HexSlice::new(&[26, 43, 60]);
        let b = [26, 43, 60].as_slice();

        assert_eq!(a, b);
    }

    #[test]
    fn eq_array() {
        let a = HexSlice::new(&[26, 43, 60]);
        let b = [26_u8, 43, 60];

        assert_eq!(*a, b);
    }

    #[test]
    fn eq_str() {
        let a = HexSlice::new(&[26, 43, 60]);
        let b = "1a2b3c";

        assert_eq!(a, b);
    }

    // [u8] -> HexSlice
    #[test]
    fn slice_as_ref() {
        let v = [1, 2, 3];
        let _: &HexSlice = v.as_slice().as_ref();
    }

    #[test]
    fn slice_as_mut() {
        let mut v = [1, 2, 3];
        let _: &mut HexSlice = v.as_mut_slice().as_mut();
    }

    // [u8; N] -> HexSlice
    #[test]
    fn array_as_ref() {
        let v = [1, 2, 3];
        let _: &HexSlice = v.as_ref();
    }

    #[test]
    fn array_as_mut() {
        let mut v = [1, 2, 3];
        let _: &mut HexSlice = v.as_mut();
    }

    // HexSlice -> [u8]
    #[test]
    fn hex_slice_as_slice_ref() {
        let x = [1, 2, 3];
        let y = HexSlice::new(&x);
        let _: &[u8] = y.as_ref();
    }

    #[test]
    fn hex_slice_as_slice_mut() {
        let mut x = [1, 2, 3];
        let y = HexSlice::new_mut(&mut x);
        let _: &mut [u8] = y.as_mut();
    }
}
