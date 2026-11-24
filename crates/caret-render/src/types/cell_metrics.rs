#[allow(unused_imports)]
use crate::prelude::*;

/// The size of one terminal cell, in pixels.
///
/// A terminal grid needs every cell the same width, so the advance of a
/// representative glyph decides the column width rather than per-glyph
/// advances.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CellMetrics {
    pub width: f32,
    pub height: f32,
    pub ascent: f32,
    pub descent: f32,
    pub underline: f32,
}
