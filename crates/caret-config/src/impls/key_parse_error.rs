#[allow(unused_imports)]
use crate::prelude::*;

impl core::fmt::Display for KeyParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "invalid key binding {:?}: {}", self.input, self.reason)
    }
}

impl std::error::Error for KeyParseError {}
