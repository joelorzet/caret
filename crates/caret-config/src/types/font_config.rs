#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct FontConfig {
    pub family: String,
    /// Tried in order when `family` has no glyph for a character.
    pub fallback: Vec<String>,
    pub size: f32,
    pub line_height: f32,
    /// Draw bold text in the bright palette slot as well as in a bold face.
    pub bold_is_bright: bool,
    /// Programming ligatures, where the family provides them.
    pub ligatures: bool,
    /// Extra pixels between cells, for fonts that sit too tight.
    pub letter_spacing: f32,
    /// Use the italic face for italic text rather than only slanting colour.
    pub italics: bool,
    /// Draw box-drawing and block characters ourselves instead of using the
    /// font's, which keeps borders seamless in fonts that space them oddly.
    pub builtin_box_drawing: bool,
}
