#[allow(unused_imports)]
use crate::prelude::*;

/// A decoded key, with the bytes it came from so an unbound key can be
/// forwarded to the child untouched.
#[derive(Clone, PartialEq, Debug)]
pub struct KeyPress {
    pub binding: KeyBinding,
    pub bytes: Vec<u8>,
}
