#[allow(unused_imports)]
use crate::prelude::*;

/// A rasterised glyph: an 8-bit coverage mask and where to put it.
///
/// Coverage rather than colour, so one raster serves every colour the cell
/// might be drawn in.
#[derive(Clone, PartialEq, Debug)]
pub struct Glyph {
    pub width: usize,
    pub height: usize,
    /// Offset from the pen position to the left edge of the bitmap.
    pub left: i32,
    /// Offset from the baseline up to the top edge of the bitmap.
    pub top: i32,
    pub coverage: Vec<u8>,
}
