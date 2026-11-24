#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Cursor {
    pub x: u16,
    pub y: u16,
    /// Set after printing into the last column with DECAWM on. The wrap is
    /// deferred until the next character, so a line that exactly fills the
    /// width does not scroll on its own.
    pub pending_wrap: bool,
}
