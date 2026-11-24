#[allow(unused_imports)]
use crate::prelude::*;

/// A family as the font picker presents it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FontFamily {
    pub name: String,
    pub monospaced: bool,
    pub faces: usize,
    pub has_bold: bool,
    pub has_italic: bool,
}
