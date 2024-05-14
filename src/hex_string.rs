use std::{
    borrow::{Borrow, BorrowMut},
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    str::FromStr,
};

use crate::{utils, Error};

#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HexString<const N: usize>([u8; N]);

impl<const N: usize> HexString<N> {
    #[must_use]
    pub fn new(v: impl Into<[u8; N]>) -> Self {
        Self(v.into())
    }

    #[must_use]
    pub fn to_lower(&self) -> String {
        self.0
            .iter()
            .copied()
            .flat_map(utils::to_hex_lower)
            .map(char::from)
            .collect()
    }

    #[must_use]
    pub fn to_upper(&self) -> String {
        self.0
            .iter()
            .copied()
            .flat_map(utils::to_hex_upper)
            .map(char::from)
            .collect()
    }

    pub fn try_parse(v: impl AsRef<str>) -> Result<Self, Error> {
        let v = v.as_ref();
        if v.len() % 2 != 0 || v.len() / 2 != N {
            return Err(Error::InvalidLength);
        }

        let mut bytes = v.bytes();
        let mut ret = [0; N];
        for v in &mut ret {
            let a = bytes.next().unwrap();
            let b = bytes.next().unwrap();

            *v = utils::from_hex([a, b]).ok_or(Error::InvalidCharacter)?;
        }

        Ok(Self::new(ret))
    }

    pub fn try_parse_lower(v: impl AsRef<str>) -> Result<Self, Error> {
        let v = v.as_ref();
        if v.len() % 2 != 0 || v.len() / 2 != N {
            return Err(Error::InvalidLength);
        }

        let mut bytes = v.bytes();
        let mut ret = [0; N];
        for v in &mut ret {
            let a = bytes.next().unwrap();
            let b = bytes.next().unwrap();

            *v = utils::from_hex_lower([a, b]).ok_or(Error::InvalidCharacter)?;
        }

        Ok(Self::new(ret))
    }

    pub fn try_parse_upper(v: impl AsRef<str>) -> Result<Self, Error> {
        let v = v.as_ref();
        if v.len() % 2 != 0 || v.len() / 2 != N {
            return Err(Error::InvalidLength);
        }

        let mut bytes = v.bytes();
        let mut ret = [0; N];
        for v in &mut ret {
            let a = bytes.next().unwrap();
            let b = bytes.next().unwrap();

            *v = utils::from_hex_upper([a, b]).ok_or(Error::InvalidCharacter)?;
        }

        Ok(Self::new(ret))
    }
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
        if s.len() % 2 != 0 || s.len() / 2 != N {
            return Err(Error::InvalidLength);
        }

        let mut bytes = s.bytes();
        let mut ret = [0; N];
        for v in &mut ret {
            let a = bytes.next().unwrap();
            let b = bytes.next().unwrap();

            *v = utils::from_hex([a, b]).ok_or(Error::InvalidCharacter)?;
        }

        Ok(Self::new(ret))
    }
}

impl<const N: usize> From<[u8; N]> for HexString<N> {
    fn from(value: [u8; N]) -> Self {
        Self::new(value)
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
            let mut bytes = other.bytes();
            for v in &self.0 {
                let a = bytes.next().unwrap();
                let b = bytes.next().unwrap();

                let Some(w) = utils::from_hex([a, b]) else {
                    return false;
                };

                if *v != w {
                    return false;
                }
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
