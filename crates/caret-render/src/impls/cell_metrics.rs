#[allow(unused_imports)]
use crate::prelude::*;

impl CellMetrics {
    /// Cell size from a font's own metrics and the configured line height.
    ///
    /// `advance` is the width of a representative glyph in pixels at the
    /// requested size; `line_height` multiplies the font's natural line
    /// spacing, and `letter_spacing` is added to the column width.
    pub fn new(
        advance: f32,
        ascent: f32,
        descent: f32,
        line_height: f32,
        letter_spacing: f32,
    ) -> CellMetrics {
        let natural = ascent + descent;
        CellMetrics {
            width: (advance + letter_spacing).max(1.0),
            height: (natural * line_height.max(0.5)).max(1.0),
            ascent,
            descent,
            underline: ascent + descent * 0.4,
        }
    }

    /// Columns and rows that fit in a window of `width` x `height` pixels,
    /// after `padding` on every side.
    pub fn grid_size(&self, width: f32, height: f32, padding: f32) -> (u16, u16) {
        let usable_w = (width - padding * 2.0).max(0.0);
        let usable_h = (height - padding * 2.0).max(0.0);
        let cols = (usable_w / self.width).floor().max(1.0);
        let rows = (usable_h / self.height).floor().max(1.0);
        (
            cols.min(u16::MAX as f32) as u16,
            rows.min(u16::MAX as f32) as u16,
        )
    }
}
