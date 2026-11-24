#[allow(unused_imports)]
use crate::prelude::*;

use caret_render::GlyphStyle;

use crate::impls::window::paint::Painter;

/// Above this an alpha hides the glyph underneath, so it is redrawn on top.
const NEARLY_OPAQUE: u8 = 200;

impl Painter {
    /// Draws the cursor over the cell it sits on.
    ///
    /// A filled block hides the character underneath, so that character is
    /// drawn again in the cursor's text colour; a translucent cursor leaves it
    /// showing and does not.
    pub fn draw_cursor(
        &mut self,
        canvas: &mut Canvas,
        cursor: CursorPaint,
        under: Option<SurfaceCell>,
    ) {
        if cursor.alpha == 0 {
            return;
        }
        let (cell_w, cell_h) = self.cell;
        let left = self.padding + cursor.at.0 as usize * cell_w;
        let top = self.chrome + self.padding + cursor.at.1 as usize * cell_h;
        let stroke = (cell_h / 10).max(2);

        let rect = match cursor.shape {
            CursorShape::Block | CursorShape::Hollow => (left, top, cell_w, cell_h),
            CursorShape::Bar => (left, top, stroke, cell_h),
            CursorShape::Underline => (left, top + cell_h - stroke, cell_w, stroke),
        };

        if cursor.shape == CursorShape::Hollow {
            self.outline_cursor(canvas, rect, cursor.colour, cursor.alpha);
            return;
        }

        let (x, y, w, h) = rect;
        for row in y..y + h {
            for col in x..x + w {
                canvas.blend(col, row, cursor.colour, cursor.alpha);
            }
        }

        let covers_cell = cursor.shape == CursorShape::Block && cursor.alpha > NEARLY_OPAQUE;
        if let Some(cell) = under.filter(|_| covers_cell) {
            if cell.ch != ' ' && cell.ch != '\0' {
                let style = GlyphStyle::of(
                    cell.flags.contains(Flags::BOLD),
                    cell.flags.contains(Flags::ITALIC),
                );
                self.blit(
                    canvas,
                    (left as i32, top as i32),
                    cell.ch,
                    cursor.text,
                    style,
                );
            }
        }
    }

    fn outline_cursor(
        &mut self,
        canvas: &mut Canvas,
        rect: (usize, usize, usize, usize),
        colour: Rgb,
        alpha: u8,
    ) {
        let (x, y, w, h) = rect;
        let edge = (h / 12).max(1);
        for row in y..y + h {
            let on_edge = row < y + edge || row >= y + h - edge;
            for col in x..x + w {
                if on_edge || col < x + edge || col >= x + w - edge {
                    canvas.blend(col, row, colour, alpha);
                }
            }
        }
    }
}
