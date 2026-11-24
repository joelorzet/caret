#[allow(unused_imports)]
use crate::prelude::*;

impl GlyphStyle {
    pub const fn of(bold: bool, italic: bool) -> GlyphStyle {
        match (bold, italic) {
            (false, false) => GlyphStyle::Regular,
            (true, false) => GlyphStyle::Bold,
            (false, true) => GlyphStyle::Italic,
            (true, true) => GlyphStyle::BoldItalic,
        }
    }

    /// Faces to try in order, so a family without an italic still draws.
    pub const fn fallbacks(self) -> &'static [GlyphStyle] {
        match self {
            GlyphStyle::Regular => &[GlyphStyle::Regular],
            GlyphStyle::Bold => &[GlyphStyle::Bold, GlyphStyle::Regular],
            GlyphStyle::Italic => &[GlyphStyle::Italic, GlyphStyle::Regular],
            GlyphStyle::BoldItalic => &[
                GlyphStyle::BoldItalic,
                GlyphStyle::Bold,
                GlyphStyle::Italic,
                GlyphStyle::Regular,
            ],
        }
    }
}
