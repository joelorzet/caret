#[allow(unused_imports)]
use crate::prelude::*;

/// The hand-authored part of a theme. Everything else is derived.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ThemeSpec {
    pub name: &'static str,
    pub description: &'static str,
    pub appearance: Appearance,
    pub bg: Rgb,
    pub fg: Rgb,
    pub cursor: Rgb,
    pub selection_bg: Rgb,
    pub accent: Rgb,
    pub normal: [Rgb; 8],
    pub bright: [Rgb; 8],
}
