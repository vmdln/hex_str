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
    ($owned:tt, $borrowed:tt, $display_fn:expr, $comparison_fn:expr, $(($other_owned:tt, $other_borrowed:tt)),*) => {
        #[repr(transparent)]
        #[derive(PartialEq, Eq, Clone, Hash)]
        pub struct $owned<const N: usize>([u8; N]);

        impl<const N: usize> $owned<N> {
            #[must_use]
            pub fn new(v: impl Into<[u8; N]>) -> Self {
                Self(v.into())
            }

            paste::paste! {
                #[must_use]
                pub fn [<as_ $borrowed:snake>](&self) -> &$borrowed<N> {
                    self.as_ref()
                }

                #[must_use]
                pub fn [<as_mut_ $borrowed:snake>](&mut self) -> &mut $borrowed<N> {
                    self.as_mut()
                }

                $(
                    #[must_use]
                    pub fn [<to $other_owned:snake>](&self) -> $other_owned<N> {
                        $other_owned::new(self.0.clone())
                    }

                    #[must_use]
                    pub fn [<into_ $other_owned:snake>](self) -> $other_owned<N> {
                        $other_owned::new(self.0)
                    }
                )*
            }
        }

        impl<const N: usize> Display for $owned<N> {
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

        impl<const N: usize> Debug for $owned<N> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($owned))
                    .field("inner", &self.to_string())
                    .field("n", &N)
                    .finish()
            }
        }

        impl<const N: usize> From<[u8; N]> for $owned<N> {
            fn from(value: [u8; N]) -> Self {
                Self::new(value)
            }
        }

        impl<const N: usize> From<$owned<N>> for [u8; N] {
            fn from(value: $owned<N>) -> Self {
                value.0
            }
        }

        $(
            impl<const N: usize> From<$owned<N>> for $other_owned<N> {
                fn from(value: $owned<N>) -> Self {
                    Self::new(value.0)
                }
            }
        )*

        impl<'a, const N: usize> TryFrom<&'a str> for $owned<N> {
            type Error = Error<&'a str, FromStrError>;

            fn try_from(value: &'a str) -> Result<Self, Self::Error> {
                value.parse().map_err(|kind| Error::new(value, kind))
            }
        }

        impl<const N: usize> TryFrom<String> for $owned<N> {
            type Error = Error<String, FromStrError>;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                value.parse().map_err(|kind| Error::new(value, kind))
            }
        }

        impl<const N: usize> FromStr for $owned<N> {
            type Err = FromStrError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if s.len() % 2 != 0 || s.len() / 2 != N {
                    return Err(FromStrError::InvalidLength);
                }

                let mut bytes = s.bytes();
                let mut ret = [0; N];
                for v in &mut ret {
                    let a = bytes.next().unwrap();
                    let b = bytes.next().unwrap();

                    *v = utils::from_hex([a, b]).ok_or(FromStrError::InvalidCharacter)?;
                }

                Ok(Self::new(ret))
            }
        }

        impl<const N: usize> PartialEq<[u8; N]> for $owned<N> {
            fn eq(&self, other: &[u8; N]) -> bool {
                self.0 == *other
            }
        }

        impl<const N: usize> PartialEq<&[u8; N]> for $owned<N> {
            fn eq(&self, other: &&[u8; N]) -> bool {
                self.0 == **other
            }
        }

        impl<const N: usize> PartialEq<Vec<u8>> for $owned<N> {
            fn eq(&self, other: &Vec<u8>) -> bool {
                self.0 == **other
            }
        }

        impl<const N: usize> PartialEq<[u8]> for $owned<N> {
            fn eq(&self, other: &[u8]) -> bool {
                self.0 == other
            }
        }

        impl<const N: usize> PartialEq<&[u8]> for $owned<N> {
            fn eq(&self, other: &&[u8]) -> bool {
                self.0 == *other
            }
        }

        impl<const N: usize> PartialEq<str> for $owned<N> {
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

        impl<const N: usize> PartialEq<&str> for $owned<N> {
            fn eq(&self, other: &&str) -> bool {
                self == *other
            }
        }

        impl<const N: usize> PartialEq<String> for $owned<N> {
            fn eq(&self, other: &String) -> bool {
                self == other.as_str()
            }
        }

        impl<const N: usize> Deref for $owned<N> {
            type Target = $borrowed<N>;

            fn deref(&self) -> &Self::Target {
                self.as_ref()
            }
        }

        impl<const N: usize> DerefMut for $owned<N> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.as_mut()
            }
        }

        impl<const N: usize> AsRef<$owned<N>> for $owned<N> {
            fn as_ref(&self) -> &$owned<N> {
                self
            }
        }

        impl<const N: usize> AsMut<$owned<N>> for $owned<N> {
            fn as_mut(&mut self) -> &mut $owned<N> {
                self
            }
        }

        $(
            impl<const N: usize> AsRef<$other_borrowed<N>> for $owned<N> {
                fn as_ref(&self) -> &$other_borrowed<N> {
                    $other_borrowed::new(&self.0)
                }
            }

            impl<const N: usize> AsMut<$other_borrowed<N>> for $owned<N> {
                fn as_mut(&mut self) -> &mut $other_borrowed<N> {
                    $other_borrowed::new_mut(&mut self.0)
                }
            }
        )*

        impl<const N: usize> AsRef<[u8; N]> for $owned<N> {
            fn as_ref(&self) -> &[u8; N] {
                &self.0
            }
        }

        impl<const N: usize> AsMut<[u8; N]> for $owned<N> {
            fn as_mut(&mut self) -> &mut [u8; N] {
                &mut self.0
            }
        }

        impl<const N: usize> AsRef<[u8]> for $owned<N> {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }

        impl<const N: usize> AsMut<[u8]> for $owned<N> {
            fn as_mut(&mut self) -> &mut [u8] {
                &mut self.0
            }
        }

        #[cfg(feature = "serde")]
        impl<'de, const N: usize> serde::Deserialize<'de> for $owned<N> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct Visitor<const O: usize>;

                impl<'de, const O: usize> serde::de::Visitor<'de> for Visitor<O> {
                    type Value = $owned<O>;

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
        impl<const N: usize> serde::Serialize for $owned<N> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.to_string().serialize(serializer)
            }
        }

        #[cfg(feature = "rand")]
        impl<const N: usize> rand::distributions::Distribution<$owned<N>> for rand::distributions::Standard {
            fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> $owned<N> {
                $owned::new(rng.gen::<[u8; N]>())
            }
        }

        #[repr(transparent)]
        #[derive(PartialEq, Eq, Hash)]
        pub struct $borrowed<const N: usize>([u8; N]);

        impl<const N: usize> $borrowed<N> {
            // unfortunately, there is no impl AsRef<[u8; N]> for [u8; N]
            #[must_use]
            pub fn new(v: &[u8; N]) -> &Self
            {
                unsafe { &*(v as *const [u8; N]).cast() }
            }

            #[must_use]
            pub fn new_mut(v: &mut [u8; N]) -> &mut Self
            {
                unsafe { &mut *(v as *mut [u8; N]).cast() }
            }

            paste::paste! {
                #[must_use]
                pub fn [<to_ $owned:snake>](&self) -> $owned<N> {
                    $owned::new(self.0.clone())
                }

                $(
                    #[must_use]
                    pub fn [<as_ $other_borrowed:snake>](&self) -> &$other_borrowed<N> {
                        self.as_ref()
                    }

                    #[must_use]
                    pub fn [<as_mut_ $other_borrowed:snake>](&mut self) -> &mut $other_borrowed<N> {
                        self.as_mut()
                    }

                    #[must_use]
                    pub fn [<to_ $other_owned:snake>](&self) -> $other_owned<N> {
                        $other_owned::new(self.0.clone())
                    }
                )*
            }

        }

        impl<const N: usize> Display for $borrowed<N> {
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

        impl<const N: usize> Debug for $borrowed<N> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($borrowed))
                    .field("inner", &self.to_string())
                    .finish()
            }
        }

        impl<const N: usize> PartialEq<$owned<N>> for $borrowed<N> {
            fn eq(&self, other: &$owned<N>) -> bool {
                self.0 == other.0
            }
        }

        impl<const N: usize> PartialEq<[u8; N]> for $borrowed<N> {
            fn eq(&self, other: &[u8; N]) -> bool {
                self.0 == *other
            }
        }

        impl<const N: usize> PartialEq<&[u8; N]> for $borrowed<N> {
            fn eq(&self, other: &&[u8; N]) -> bool {
                self.0 == **other
            }
        }

        impl<const N: usize> PartialEq<Vec<u8>> for $borrowed<N> {
            fn eq(&self, other: &Vec<u8>) -> bool {
                self.0 == **other
            }
        }

        impl<const N: usize> PartialEq<[u8]> for $borrowed<N> {
            fn eq(&self, other: &[u8]) -> bool {
                &self.0 == other
            }
        }

        impl<const N: usize> PartialEq<&[u8]> for $borrowed<N> {
            fn eq(&self, other: &&[u8]) -> bool {
                &self.0 == *other
            }
        }

        impl<const N: usize> PartialEq<str> for $borrowed<N> {
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

        impl<const N: usize> PartialEq<&str> for $borrowed<N> {
            fn eq(&self, other: &&str) -> bool {
                self == *other
            }
        }

        impl<const N: usize> PartialEq<String> for $borrowed<N> {
            fn eq(&self, other: &String) -> bool {
                self == other.as_str()
            }
        }

        impl<const N: usize> AsRef<$borrowed<N>> for [u8; N] {
            fn as_ref(&self) -> &$borrowed<N> {
                $borrowed::new(self)
            }
        }

        impl<const N: usize> AsMut<$borrowed<N>> for [u8; N] {
            fn as_mut(&mut self) -> &mut $borrowed<N> {
                $borrowed::new_mut(self)
            }
        }

        impl<const N: usize> AsRef<$borrowed<N>> for $owned<N> {
            fn as_ref(&self) -> &$borrowed<N> {
                $borrowed::new(&self.0)
            }
        }

        impl<const N: usize> AsMut<$borrowed<N>> for $owned<N> {
            fn as_mut(&mut self) -> &mut $borrowed<N> {
                $borrowed::new_mut(&mut self.0)
            }
        }

        impl<const N: usize> AsRef<[u8]> for $borrowed<N> {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }

        impl<const N: usize> AsMut<[u8]> for $borrowed<N> {
            fn as_mut(&mut self) -> &mut [u8] {
                &mut self.0
            }
        }

        impl<const N: usize> AsRef<[u8; N]> for $borrowed<N> {
            fn as_ref(&self) -> &[u8; N] {
                &self.0
            }
        }

        impl<const N: usize> AsMut<[u8; N]> for $borrowed<N> {
            fn as_mut(&mut self) -> &mut [u8; N] {
                &mut self.0
            }
        }

        $(
            impl<const N: usize> AsRef<$other_borrowed<N>> for $borrowed<N> {
                fn as_ref(&self) -> &$other_borrowed<N> {
                    $other_borrowed::new(&self.0)
                }
            }

            impl<const N: usize> AsMut<$other_borrowed<N>> for $borrowed<N> {
                fn as_mut(&mut self) -> &mut $other_borrowed<N> {
                    $other_borrowed::new_mut(&mut self.0)
                }
            }
        )*

        impl<const N: usize> ToOwned for $borrowed<N> {
            type Owned = $owned<N>;

            fn to_owned(&self) -> Self::Owned {
                $owned::new(self.0.clone())
            }
        }

        impl<const N: usize> Borrow<$borrowed<N>> for $owned<N> {
            fn borrow(&self) -> &$borrowed<N> {
                self.as_ref()
            }
        }

        impl<const N: usize> BorrowMut<$borrowed<N>> for $owned<N> {
            fn borrow_mut(&mut self) -> &mut $borrowed<N> {
                self.as_mut()
            }
        }

        impl<const N: usize> Deref for $borrowed<N> {
            type Target = [u8; N];

            fn deref(&self) -> &Self::Target {
                self.as_ref()
            }
        }

        impl<const N: usize> DerefMut for $borrowed<N> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.as_mut()
            }
        }
    };
}

impl_h!(
    HexStringN,
    HexStrN,
    utils::to_hex_lower,
    utils::from_hex,
    (HexStringNLower, HexStrNLower),
    (HexStringNUpper, HexStrNUpper)
);

impl_h!(
    HexStringNLower,
    HexStrNLower,
    utils::to_hex_lower,
    utils::from_hex_lower,
    (HexStringN, HexStrN),
    (HexStringNUpper, HexStrNUpper)
);
impl_h!(
    HexStringNUpper,
    HexStrNUpper,
    utils::to_hex_upper,
    utils::from_hex_upper,
    (HexStringN, HexStrN),
    (HexStringNLower, HexStrNLower)
);
