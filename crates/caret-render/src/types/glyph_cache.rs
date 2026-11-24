#[allow(unused_imports)]
use crate::prelude::*;

use std::collections::HashMap;

/// Rasterises glyphs once and keeps them.
///
/// A terminal redraws the same few hundred characters constantly, so caching
/// the coverage masks turns rendering into a blend rather than a rasterise.
pub struct GlyphCache {
    pub(crate) faces: HashMap<GlyphStyle, LoadedFace>,
    pub(crate) size: f32,
    pub(crate) metrics: CellMetrics,
    pub(crate) glyphs: HashMap<(char, GlyphStyle), Glyph>,
    /// Glyphs named by the font's own index rather than by a character,
    /// which is how a ligature is named: it has no character of its own.
    pub(crate) by_glyph: HashMap<(u16, GlyphStyle), Glyph>,
    /// Runs the font has already been asked to shape, per face. A row is a
    /// handful of runs and the same rows are drawn again and again, so this
    /// is what keeps shaping off the frame.
    pub(crate) shaped: HashMap<GlyphStyle, HashMap<String, Vec<Piece>>>,
    /// Returned for characters no face can draw.
    pub(crate) blank: Glyph,
}
