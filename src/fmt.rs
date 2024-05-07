use std::{borrow::Cow, fmt::Display};

use crate::utils;

pub struct Lower<'a>(pub(crate) Cow<'a, [u8]>);

impl<'a> Display for Lower<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tmp: String = self
            .0
            .iter()
            .copied()
            .flat_map(utils::to_hex_lower)
            .map(char::from)
            .collect();

        f.write_str(&tmp)
    }
}

pub struct Upper<'a>(pub(crate) Cow<'a, [u8]>);

impl<'a> Display for Upper<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tmp: String = self
            .0
            .iter()
            .copied()
            .flat_map(utils::to_hex_upper)
            .map(char::from)
            .collect();

        f.write_str(&tmp)
    }
}
