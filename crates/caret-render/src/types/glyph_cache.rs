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
    /// Returned for characters no face can draw.
    pub(crate) blank: Glyph,
}
