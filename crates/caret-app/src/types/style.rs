#[allow(unused_imports)]
use crate::prelude::*;

/// Colours and attributes for a run of text.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Style {
    pub fg: Rgb,
    pub bg: Rgb,
    pub flags: Flags,
}
