use std::{
    borrow::{Borrow, BorrowMut},
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    str::FromStr,
};

use crate::{
    error::{Error, FromStrError},
    utils,
};

macro_rules! impl_h {
    ($owned:ident, $borrowed:ident, $display_fn:expr, $comparison_fn:expr, $(($other_owned:ident, $other_borrowed:ident)),*) => {
        #[repr(transparent)]
        #[derive(PartialEq, Eq, Clone, Hash)]
        pub struct $owned(Vec<u8>);

        impl $owned {
            #[must_use]
            pub fn new(v: impl Into<Vec<u8>>) -> Self {
                Self(v.into())
            }

            pub fn push(&mut self, v: impl Into<u8>) {
                self.0.push(v.into())
            }

            #[must_use]
            pub fn pop(&mut self) -> Option<u8> {
                self.0.pop()
            }

            paste::paste! {
                #[must_use]
                pub fn [<as_ $borrowed:snake>](&self) -> &$borrowed {
                    self.as_ref()
                }

                #[must_use]
                pub fn [<as_mut_ $borrowed:snake>](&mut self) -> &mut $borrowed {
                    self.as_mut()
                }

                $(
                    #[must_use]
                    pub fn [<to $other_owned:snake>](&self) -> $other_owned {
                        $other_owned::new(self.0.clone())
                    }

                    #[must_use]
                    pub fn [<into_ $other_owned:snake>](self) -> $other_owned {
                        $other_owned::new(self.0)
                    }
                )*
            }

            #[must_use]
            pub fn as_bytes(&self) -> &[u8] {
                self.as_ref()
            }

            #[must_use]
            pub fn as_mut_vec(&mut self) -> &mut Vec<u8> {
                self.as_mut()
            }
        }

        impl Display for $owned {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let tmp: String = self
                    .0
                    .iter()
                    .copied()
                    .flat_map($display_fn)
                    .map(char::from)
                    .collect();

                Display::fmt(&tmp, f)
            }
        }

        impl Debug for $owned {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($owned))
                    .field("inner", &self.to_string())
                    .finish()
            }
        }

        impl From<&[u8]> for $owned {
            fn from(value: &[u8]) -> Self {
                Self::new(value)
            }
        }

        impl<const N: usize> From<&[u8; N]> for $owned {
            fn from(value: &[u8; N]) -> Self {
                Self::new(value)
            }
        }

        impl From<Vec<u8>> for $owned {
            fn from(value: Vec<u8>) -> Self {
                Self::new(value)
            }
        }

        impl From<$owned> for Vec<u8> {
            fn from(value: $owned) -> Self {
                value.0
            }
        }

        impl<'a> TryFrom<&'a str> for $owned {
            type Error = Error<&'a str, FromStrError>;

            fn try_from(value: &'a str) -> Result<Self, Self::Error> {
                value.parse().map_err(|kind| Error::new(value, kind))
            }
        }

        impl TryFrom<String> for $owned {
            type Error = Error<String, FromStrError>;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                value.parse().map_err(|kind| Error::new(value, kind))
            }
        }

        impl FromStr for $owned {
            type Err = FromStrError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if s.len() % 2 != 0 {
                    return Err(FromStrError::InvalidLength);
                }

                let mut bytes = s.bytes();
                let mut ret = Vec::with_capacity(s.len() / 2);
                while let Some(a) = bytes.next() {
                    let b = bytes.next().unwrap();

                    let v = utils::from_hex([a, b]).ok_or(FromStrError::InvalidCharacter)?;
                    ret.push(v);
                }

                Ok(Self::new(ret))
            }
        }

        impl<const N: usize> PartialEq<[u8; N]> for $owned {
            fn eq(&self, other: &[u8; N]) -> bool {
                self.0 == other
            }
        }

        impl<const N: usize> PartialEq<&[u8; N]> for $owned {
            fn eq(&self, other: &&[u8; N]) -> bool {
                self.0 == *other
            }
        }

        impl PartialEq<Vec<u8>> for $owned {
            fn eq(&self, other: &Vec<u8>) -> bool {
                self.0 == *other
            }
        }

        impl PartialEq<[u8]> for $owned {
            fn eq(&self, other: &[u8]) -> bool {
                self.0 == other
            }
        }

        impl PartialEq<&[u8]> for $owned {
            fn eq(&self, other: &&[u8]) -> bool {
                self.0 == *other
            }
        }

        impl PartialEq<str> for $owned {
            fn eq(&self, other: &str) -> bool {
                match other.len() {
                    v if v % 2 != 0 || v / 2 != self.0.len() => false,
                    _ => {
                        let mut bytes = other.bytes();
                        for v in &self.0 {
                            let a = bytes.next().unwrap();
                            let b = bytes.next().unwrap();

                            let Some(w) = $comparison_fn([a, b]) else {
                                return false;
                            };

                            if *v != w {
                                return false;
                            }
                        }

                        true
                    }
                }
            }
        }

        impl PartialEq<&str> for $owned {
            fn eq(&self, other: &&str) -> bool {
                self == *other
            }
        }

        impl PartialEq<String> for $owned {
            fn eq(&self, other: &String) -> bool {
                self == other.as_str()
            }
        }

        impl Deref for $owned {
            type Target = $borrowed;

            fn deref(&self) -> &Self::Target {
                self.as_ref()
            }
        }

        impl DerefMut for $owned {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.as_mut()
            }
        }

        impl AsRef<$owned> for $owned {
            fn as_ref(&self) -> &$owned {
                self
            }
        }

        impl AsMut<$owned> for $owned {
            fn as_mut(&mut self) -> &mut $owned {
                self
            }
        }

        impl AsRef<[u8]> for $owned {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }

        impl AsMut<Vec<u8>> for $owned {
            fn as_mut(&mut self) -> &mut Vec<u8> {
                &mut self.0
            }
        }

        impl FromIterator<u8> for $owned {
            fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
                let v: Vec<u8> = iter.into_iter().collect();
                Self::new(v)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for $owned {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = $owned;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        formatter.write_fmt(format_args!("hex string"))
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
        impl serde::Serialize for $owned {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.to_string().serialize(serializer)
            }
        }

        #[repr(transparent)]
        #[derive(PartialEq, Eq, Hash)]
        pub struct $borrowed([u8]);

        impl $borrowed {
            #[must_use]
            pub fn new<T>(v: &T) -> &Self
            where
                T: AsRef<[u8]> + ?Sized,
            {
                unsafe { &*(v.as_ref() as *const [u8] as *const $borrowed) }
            }

            #[must_use]
            pub fn new_mut<T>(v: &mut T) -> &mut Self
            where
                T: AsMut<[u8]> + ?Sized,
            {
                unsafe { &mut *(v.as_mut() as *mut [u8] as *mut $borrowed) }
            }

            paste::paste! {
                #[must_use]
                pub fn [<to_ $owned:snake>](&self) -> $owned {
                    $owned::new(&self.0)
                }

                $(
                    #[must_use]
                    pub fn [<as_ $other_borrowed:snake>](&self) -> &$other_borrowed {
                        $other_borrowed::new(self)
                    }

                    #[must_use]
                    pub fn [<as_mut_ $other_borrowed:snake>](&mut self) -> &mut $other_borrowed {
                        $other_borrowed::new_mut(self)
                    }

                    #[must_use]
                    pub fn [<to_ $other_owned:snake>](&self) -> $other_owned {
                        $other_owned::new(&self.0)
                    }
                )*
            }
        }

        impl Display for $borrowed {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let tmp: String = self
                    .0
                    .iter()
                    .copied()
                    .flat_map($display_fn)
                    .map(char::from)
                    .collect();

                Display::fmt(&tmp, f)
            }
        }

        impl Debug for $borrowed {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($borrowed))
                    .field("inner", &self.to_string())
                    .finish()
            }
        }

        impl PartialEq<$owned> for $borrowed {
            fn eq(&self, other: &$owned) -> bool {
                &self.0 == other.0
            }
        }

        impl PartialEq<Vec<u8>> for $borrowed {
            fn eq(&self, other: &Vec<u8>) -> bool {
                &self.0 == *other
            }
        }

        impl PartialEq<[u8]> for $borrowed {
            fn eq(&self, other: &[u8]) -> bool {
                &self.0 == other
            }
        }

        impl PartialEq<&[u8]> for $borrowed {
            fn eq(&self, other: &&[u8]) -> bool {
                &self.0 == *other
            }
        }

        impl PartialEq<str> for $borrowed {
            fn eq(&self, other: &str) -> bool {
                match other.len() {
                    v if v % 2 != 0 => false,
                    v if v / 2 != self.0.len() => false,
                    _ => {
                        let mut bytes = other.bytes();
                        for v in &self.0 {
                            let a = bytes.next().unwrap();
                            let b = bytes.next().unwrap();

                            let Some(w) = $comparison_fn([a, b]) else {
                                return false;
                            };

                            if *v != w {
                                return false;
                            }
                        }

                        true
                    }
                }
            }
        }

        impl PartialEq<&str> for $borrowed {
            fn eq(&self, other: &&str) -> bool {
                self == *other
            }
        }

        impl PartialEq<String> for $borrowed {
            fn eq(&self, other: &String) -> bool {
                self == other.as_str()
            }
        }

        impl AsRef<$borrowed> for $owned {
            fn as_ref(&self) -> &$borrowed {
                $borrowed::new(&self.0)
            }
        }

        impl AsMut<$borrowed> for $owned {
            fn as_mut(&mut self) -> &mut $borrowed {
                $borrowed::new_mut(&mut self.0)
            }
        }

        impl AsRef<$borrowed> for [u8] {
            fn as_ref(&self) -> &$borrowed {
                $borrowed::new(self)
            }
        }

        impl AsMut<$borrowed> for [u8] {
            fn as_mut(&mut self) -> &mut $borrowed {
                $borrowed::new_mut(self)
            }
        }

        impl AsRef<[u8]> for $borrowed {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }

        impl AsMut<[u8]> for $borrowed {
            fn as_mut(&mut self) -> &mut [u8] {
                &mut self.0
            }
        }

        $(
            impl From<$owned> for $other_owned {
                fn from(value: $owned) -> Self {
                    Self::new(value.0)
                }
            }

            impl AsRef<$other_borrowed> for $owned {
                fn as_ref(&self) -> &$other_borrowed {
                    $other_borrowed::new(&self.0)
                }
            }

            impl AsMut<$other_borrowed> for $owned {
                fn as_mut(&mut self) -> &mut $other_borrowed {
                    $other_borrowed::new_mut(&mut self.0)
                }
            }

            impl AsRef<$other_borrowed> for $borrowed {
                fn as_ref(&self) -> &$other_borrowed {
                    $other_borrowed::new(&self.0)
                }
            }

            impl AsMut<$other_borrowed> for $borrowed {
                fn as_mut(&mut self) -> &mut $other_borrowed {
                    $other_borrowed::new_mut(&mut self.0)
                }
            }
        )*

        impl ToOwned for $borrowed {
            type Owned = $owned;

            fn to_owned(&self) -> Self::Owned {
                $owned::new(self.0.to_vec())
            }
        }

        impl Borrow<$borrowed> for $owned {
            fn borrow(&self) -> &$borrowed {
                self.as_ref()
            }
        }

        impl BorrowMut<$borrowed> for $owned {
            fn borrow_mut(&mut self) -> &mut $borrowed {
                self.as_mut()
            }
        }

        impl Deref for $borrowed {
            type Target = [u8];

            fn deref(&self) -> &Self::Target {
                self.as_ref()
            }
        }

        impl DerefMut for $borrowed {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.as_mut()
            }
        }
    };
}

impl_h!(
    HexString,
    HexStr,
    utils::to_hex_lower,
    utils::from_hex,
    (HexStringLower, HexStrLower),
    (HexStringUpper, HexStrUpper)
);
impl_h!(
    HexStringLower,
    HexStrLower,
    utils::to_hex_lower,
    utils::from_hex_lower,
    (HexString, HexStr),
    (HexStringUpper, HexStrUpper)
);
impl_h!(
    HexStringUpper,
    HexStrUpper,
    utils::to_hex_upper,
    utils::from_hex_upper,
    (HexString, HexStr),
    (HexStringLower, HexStrLower)
);
