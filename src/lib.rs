#![warn(clippy::pedantic)]
#![deny(rust_2018_idioms, unused, future_incompatible, nonstandard_style)]
#![allow(clippy::module_name_repetitions, clippy::module_inception)]

pub mod error;
pub mod fmt;
mod hex_string;
mod hex_string_n;
mod utils;

pub use hex_string::HexString;
pub use hex_string_n::HexStringN;
