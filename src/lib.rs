#![warn(clippy::pedantic)]
#![deny(rust_2018_idioms, unused, future_incompatible, nonstandard_style)]

//! Handle and parse hex strings of constant and variable lengths
//!
//! ## Example:
//! Example hex string, an md5 of an empty file:
//! ```text
//! d41d8cd98f00b204e9800998ecf8427e
//! ```
//! ```
//! use hex_str::{HexString, HexStringN};
//!
//! let s = "d41d8cd98f00b204e9800998ecf8427e";
//!
//! // constant length, encoded in the type system
//! let u = HexStringN::<16>::try_parse(s).unwrap();
//! assert_eq!(u, "d41d8cd98f00b204e9800998ecf8427e");
//!
//! // variable length
//! let v = HexString::try_parse(s).unwrap();
//! assert_eq!(v, "d41d8cd98f00b204e9800998ecf8427e");
//! ```

//! ## Feature flags:
//! - `serde` - adds the ability to serialize, and deserialize [`HexString`]'s, and [`HexStringN`]'s using `serde`.
//! - `rand` - adds implementation of `rand`'s [`Standard`](https://docs.rs/rand/0.8.4/rand/distributions/struct.Standard.html)
//!   distribution, which enables random generation of [`HexStringN`]'s directly.
//!
//! #### Using `serde` feature:
//! ```
//! #[cfg(feature = "serde")]
//! {
//!     use hex_str::HexStringN;
//!     use serde::{Deserialize, Serialize};
//!
//!     #[derive(Deserialize, Serialize)]
//!     struct Example {
//!         md5: HexStringN<16>,
//!     }
//!
//!     let s = r#"
//!     {
//!         "md5": "d41d8cd98f00b204e9800998ecf8427e"
//!     }
//!     "#;
//!
//!     let example: Example = serde_json::from_str(s).unwrap();
//!     assert_eq!(example.md5, "d41d8cd98f00b204e9800998ecf8427e");
//!
//!     serde_json::to_string(&example).unwrap();
//! }
//! ```
//!
//! #### Using `rand` feature:
//! ```
//! #[cfg(feature = "rand")]
//! {
//!     use hex_str::HexStringN;
//!
//!     let _: HexStringN<16> = rand::random();
//! }
//! ```

mod error;
mod hex_string;
mod hex_string_n;
mod utils;

pub use error::{HexStringError, HexStringNError};
pub use hex_string::HexString;
pub use hex_string_n::HexStringN;
