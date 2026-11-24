#[allow(unused_imports)]
use crate::prelude::*;

/// A loaded face: the rasteriser, plus the bytes needed to look a character
/// up for itself.
///
/// `fontdue`'s own cmap lookup picks the wrong glyph for several Latin-1
/// characters in at least Menlo (`é`, `ü`, `°` and `·` all come back as
/// something else), so character-to-glyph goes through `ttf-parser` and only
/// the rasterising is left to `fontdue`.
pub struct LoadedFace {
    pub(crate) font: fontdue::Font,
    pub(crate) bytes: std::sync::Arc<Vec<u8>>,
    pub(crate) index: u32,
}
