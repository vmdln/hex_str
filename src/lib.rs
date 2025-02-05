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
//! use hex_str::{HexVector, HexArray};
//!
//! let s = "d41d8cd98f00b204e9800998ecf8427e";
//!
//! // constant length, encoded in the type system
//! let u = HexArray::<16>::try_parse(s).unwrap();
//! assert_eq!(u, "d41d8cd98f00b204e9800998ecf8427e");
//!
//! // variable length
//! let v = HexVector::try_parse(s).unwrap();
//! assert_eq!(v, "d41d8cd98f00b204e9800998ecf8427e");
//! ```

//! ## Stack vs. Heap Allocation
//! [`HexArray`] is a `#[repr(transparent)]` wrapper around an array. As a result, instances with large values of `N` may risk stack overflow. Simply boxing the value does not necessarily prevent this issue, as Rust does not currently support placement new.  
//!
//! When allocating on the heap, Rust first places the object on the stack before copying it to the heap. While compiler optimizations may sometimes elide this intermediate step, such optimizations are not guaranteed and should not be relied upon.  
//!
//! To facilitate heap allocation, boxed variants of relevant functions are provided. These functions are suffixed with `_boxed` (e.g., [`HexArray::new_boxed()`]) and ensure direct allocation on the heap, avoiding potential stack overflow concerns, and costly memcpy's which are required when moving stack allocated arrays around.

//! ## Feature flags:
//! - `serde` - adds the ability to serialize, and deserialize [`HexVector`]'s, and [`HexArray`]'s using `serde`.
//! - `rand` - adds implementation of `rand`'s [`Standard`](https://docs.rs/rand/0.8.4/rand/distributions/struct.Standard.html)
//!   distribution, which enables random generation of [`HexArray`]'s directly.
//!
//! #### Using `serde` feature:
//! ```
//! #[cfg(feature = "serde")]
//! {
//!     use hex_str::HexArray;
//!     use serde::{Deserialize, Serialize};
//!
//!     #[derive(Deserialize, Serialize)]
//!     struct Example {
//!         md5: HexArray<16>,
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
//!     use hex_str::HexArray;
//!
//!     // stack allocated
//!     let _: HexArray<16> = rand::random();
//!
//!     // heap allocated
//!     let _: Box<HexArray<1_048_576>> = rand::random();
//! }
//! ```

mod error;
mod hex_array;
mod hex_vector;
mod utils;

pub use error::{HexArrayError, HexVectorError};
pub use hex_array::HexArray;
pub use hex_vector::HexVector;
