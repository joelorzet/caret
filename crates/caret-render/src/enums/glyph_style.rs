#[allow(unused_imports)]
use crate::prelude::*;

/// Which face a glyph is drawn with.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GlyphStyle {
    Regular,
    Bold,
    Italic,
    BoldItalic,
}
