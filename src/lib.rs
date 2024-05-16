#![warn(clippy::pedantic)]
#![deny(rust_2018_idioms, unused, future_incompatible, nonstandard_style)]

//! Handle and parse hex strings of specific length.
//!
//! Example hex string, an md5 of an empty file:
//! ```text
//! d41d8cd98f00b204e9800998ecf8427e
//! ```
//!
//! ## Example
//! ```
//! use hex_str::{HexString, Error};
//!
//! // parsing
//! let s = "d41d8cd98f00b204e9800998ecf8427e";
//! let v = HexString::<16>::try_parse(s);
//! assert_eq!(v.unwrap(), "d41d8cd98f00b204e9800998ecf8427e");
//! ```

//! ## Feature flags:
//! - `serde` - adds the ability to serialize and deserialize a [`HexString`] using `serde`.
//! - `rand` - adds implementation of `rand`'s [`Standard`](https://docs.rs/rand/0.8.4/rand/distributions/struct.Standard.html)
//! distribution, which enables random generation of [`HexString`]'s directly.
//!
//! #### Using `serde` feature:
//! ```
//! #[cfg(feature = "serde")]
//! {
//!     use hex_str::HexString;
//!     use serde::{Deserialize};
//!
//!     #[derive(Deserialize)]
//!     struct Example {
//!         md5: HexString<16>,
//!     }
//!
//!     let s = r#"
//!         {
//!             "md5": "d41d8cd98f00b204e9800998ecf8427e"
//!         }
//!     "#;
//!
//!     let example: Example = serde_json::from_str(s).unwrap();
//!     assert_eq!(example.md5, "d41d8cd98f00b204e9800998ecf8427e");
//! }
//! ```
//!
//! #### Using `rand` feature:
//! ```
//! #[cfg(feature = "rand")]
//! {
//!     use hex_str::HexString;
//!
//!     let _: HexString<16> = rand::random();
//! }
//! ```

mod error;
mod hex_string;
mod utils;

pub use error::Error;
pub use hex_string::HexString;
