#[allow(unused_imports)]
use crate::prelude::*;

/// Colours for highlighted code, guaranteed to be distinguishable.
///
/// Reaching into fixed palette slots fails on a deliberately monochrome
/// theme: every slot of `phosphor-green` is a green, so code came out one
/// colour.
/// These are derived instead, and separated from each other if the theme's own
/// colours sit too close together.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SyntaxPalette {
    pub plain: Rgb,
    pub comment: Rgb,
    pub string: Rgb,
    pub number: Rgb,
    pub keyword: Rgb,
    pub type_name: Rgb,
    pub function: Rgb,
    pub property: Rgb,
}
