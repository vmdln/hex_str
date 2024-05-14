#![warn(clippy::pedantic)]
#![deny(rust_2018_idioms, unused, future_incompatible, nonstandard_style)]

mod error;
mod hex_string;
mod utils;

pub use error::Error;
pub use hex_string::HexString;
