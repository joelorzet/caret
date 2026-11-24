#[allow(unused_imports)]
use crate::prelude::*;

use caret_render::{BlockShape, BoxArms, BoxCorner, BoxGlyph, CellMetrics, GlyphCache, GlyphStyle};

/// Draws a composed frame into a 32-bit pixel buffer.
///
/// The buffer is `0x00RRGGBB`, which is what softbuffer presents.
pub struct Painter {
    pub cache: GlyphCache,
    pub cell: (usize, usize),
    pub baseline: usize,
    pub padding: usize,
    /// Draw box-drawing characters as rectangles rather than from the font.
    pub builtin_boxes: bool,
    /// Pixels reserved above the grid for chrome the painter does not own.
    pub chrome: usize,
}

impl Painter {
    pub fn new(cache: GlyphCache, padding: usize, builtin_boxes: bool) -> Painter {
        let metrics = cache.metrics();
        Painter {
            cell: (
                metrics.width.round().max(1.0) as usize,
                metrics.height.round().max(1.0) as usize,
            ),
            baseline: metrics.ascent.round().max(1.0) as usize,
            cache,
            padding,
            builtin_boxes,
            chrome: 0,
        }
    }

    pub fn metrics(&self) -> CellMetrics {
        self.cache.metrics()
    }

    /// Columns and rows that fit a window of this pixel size.
    pub fn grid_size(&self, width: usize, height: usize) -> (u16, u16) {
        let usable_w = width.saturating_sub(self.padding * 2);
        let usable_h = height
            .saturating_sub(self.chrome)
            .saturating_sub(self.padding * 2);
        (
            (usable_w / self.cell.0).clamp(1, u16::MAX as usize) as u16,
            (usable_h / self.cell.1).clamp(1, u16::MAX as usize) as u16,
        )
    }

    pub fn window_size(&self, cols: u16, rows: u16) -> (u32, u32) {
        (
            (cols as usize * self.cell.0 + self.padding * 2) as u32,
            (rows as usize * self.cell.1 + self.padding * 2 + self.chrome) as u32,
        )
    }

    /// Draws a line of text at a pixel position, clipped to `max_w`.
    ///
    /// Chrome shares the terminal's font, so it keeps the cell advance and
    /// stays aligned with the grid underneath; only the origin is free.
    pub fn text_px(
        &mut self,
        canvas: &mut Canvas,
        at: (i32, i32),
        max_w: u32,
        text: &str,
        colour: Rgb,
        bold: bool,
    ) -> i32 {
        let style = GlyphStyle::of(bold, false);
        let (cell_w, _) = self.cell;
        let fits = (max_w as usize / cell_w.max(1)).max(1);
        let count = text.chars().count();

        let mut x = at.0;
        for (index, ch) in text.chars().enumerate() {
            if index >= fits {
                break;
            }
            // The last column that fits becomes an ellipsis when there is
            // more text behind it.
            let ch = if index + 1 == fits && count > fits {
                '\u{2026}'
            } else {
                ch
            };
            self.blit(canvas, (x, at.1), ch, colour, style);
            x += cell_w as i32;
        }
        x
    }

    /// Draws one character at a pixel position, clipped to its cell.
    pub(crate) fn blit(
        &mut self,
        canvas: &mut Canvas,
        at: (i32, i32),
        ch: char,
        colour: Rgb,
        style: GlyphStyle,
    ) {
        let glyph = self.cache.glyph(ch, style);
        if glyph.width == 0 || glyph.height == 0 {
            return;
        }
        let (cell_w, cell_h) = self.cell;
        let x0 = at.0 + glyph.left;
        let y0 = at.1 + self.baseline as i32 - glyph.top;
        let (right, bottom) = (at.0 + cell_w as i32, at.1 + cell_h as i32);

        for gy in 0..glyph.height {
            let y = y0 + gy as i32;
            if y < at.1.max(0) || y >= bottom {
                continue;
            }
            for gx in 0..glyph.width {
                let x = x0 + gx as i32;
                if x < at.0.max(0) || x >= right {
                    continue;
                }
                let alpha = glyph.coverage[gy * glyph.width + gx];
                if alpha > 0 {
                    canvas.blend(x as usize, y as usize, colour, alpha);
                }
            }
        }
    }

    /// How tall the chrome strip should be for this font.
    pub fn strip_height(&self) -> usize {
        (self.cell.1 as f32 * 1.9).round() as usize
    }

    pub fn paint(
        &mut self,
        buffer: &mut [u32],
        size: (usize, usize),
        surface: &Surface,
        background: Rgb,
    ) {
        let (width, height) = size;
        buffer.fill(background.to_u32());
        let mut canvas = Canvas {
            pixels: buffer,
            width,
            height,
        };

        let (cols, rows) = surface.size();
        let (cell_w, cell_h) = self.cell;

        let origin = self.chrome + self.padding;
        for row in 0..rows {
            let top = origin + row as usize * cell_h;
            if top >= height {
                break;
            }
            for col in 0..cols {
                let Some(cell) = surface.get(col, row) else {
                    continue;
                };
                let left = self.padding + col as usize * cell_w;
                if left >= width {
                    break;
                }

                let grapheme = surface.grapheme(&cell);
                let shape = BoxGlyph::of(grapheme.base()).filter(|_| self.builtin_boxes);
                // A rounded corner carries its own background, curved to
                // match; filling the cell square first would leave the panel
                // poking out past the arc.
                if matches!(shape, Some(BoxGlyph::Arc(_))) {
                    // The arc paints its own curved background.
                } else if cell.round.is_empty() {
                    canvas.fill(left, top, (cell_w, cell_h), cell.bg);
                } else {
                    fill_rounded(&mut canvas, (left, top), (cell_w, cell_h), &cell);
                }
                let base = grapheme.base();
                let drawable =
                    base != ' ' && base != '\0' && !cell.flags.contains(Flags::WIDE_TAIL);
                if drawable {
                    // A box character from the font stops short of the cell
                    // edges, so a column of them reads as a dotted line.
                    // Drawing the shape instead makes neighbours meet.
                    match shape {
                        Some(shape) => draw_box(
                            &mut canvas,
                            (left, top),
                            (cell_w, cell_h),
                            shape,
                            cell.fg,
                            cell.bg,
                        ),
                        None => {
                            // A wide glyph is allowed the cell it advances
                            // by and the tail cell that belongs to it.
                            let wide = surface
                                .get(col + 1, row)
                                .is_some_and(|next| next.flags.contains(Flags::WIDE_TAIL));
                            let span = (cell_w * if wide { 2 } else { 1 }, cell_h);
                            // The base first, then whatever is written on
                            // it: a combining glyph advances by nothing and
                            // is drawn to sit over the character before it,
                            // so it goes at the same origin.
                            for ch in grapheme.chars() {
                                draw_glyph(
                                    &mut self.cache,
                                    &mut canvas,
                                    (left, top),
                                    span,
                                    self.baseline,
                                    ch,
                                    &cell,
                                );
                            }
                        }
                    }
                }
                if cell.flags.any_underline() {
                    canvas.fill(left, top + self.baseline + 1, (cell_w, 1), cell.fg);
                }
                if cell.flags.contains(Flags::STRIKE) {
                    let y = top + self.baseline - self.baseline / 3;
                    canvas.fill(left, y, (cell_w, 1), cell.fg);
                }
            }
        }
    }
}

/// Draws a box or block character from its geometry.
fn draw_box(
    canvas: &mut Canvas,
    origin: (usize, usize),
    size: (usize, usize),
    shape: BoxGlyph,
    colour: Rgb,
    bg: Rgb,
) {
    match shape {
        BoxGlyph::Lines(arms) => draw_arms(canvas, origin, size, arms, colour),
        BoxGlyph::Arc(corner) => draw_arc(canvas, origin, size, corner, colour, bg),
        BoxGlyph::Block(block) => draw_block(canvas, origin, size, block, colour),
    }
}

/// A rounded corner: a quarter ellipse meeting the arms of its neighbours.
///
/// The ellipse is centred on the cell corner opposite the bulge with semi-axes
/// of half the cell, so it passes exactly through the midpoints of the two
/// edges the straight arms arrive at, so the join is seamless whatever the cell
/// proportions. The curve is antialiased, which is the whole point: stepping
/// it would look worse than the right angle it replaces.
fn draw_arc(
    canvas: &mut Canvas,
    origin: (usize, usize),
    size: (usize, usize),
    corner: BoxCorner,
    colour: Rgb,
    bg: Rgb,
) {
    let (left, top) = origin;
    let (w, h) = size;
    let (wf, hf) = (w as f32, h as f32);
    let stroke = (hf / 10.0).max(1.0);

    // A circle rather than an ellipse. Fitting the curve to the cell made the
    // radius follow the cell's shape, and a cell is about twice as tall as it
    // is wide, so every corner came out stretched. The arms meet this cell at
    // its mid-lines, so the largest circle that reaches both is half the
    // narrower side.
    let radius = wf.min(hf) / 2.0;
    let (edge_x, edge_y) = (wf / 2.0, hf / 2.0);

    // Which way the box lies from this corner.
    let (sx, sy) = match corner {
        BoxCorner::TopLeft => (1.0, 1.0),
        BoxCorner::TopRight => (-1.0, 1.0),
        BoxCorner::BottomRight => (-1.0, -1.0),
        BoxCorner::BottomLeft => (1.0, -1.0),
    };
    let centre = (edge_x + sx * radius, edge_y + sy * radius);

    // The centre of the cell's own rounded corner, a radius in from the two
    // sides that face away from the box.
    let outer = (
        if sx > 0.0 { radius } else { wf - radius },
        if sy > 0.0 { radius } else { hf - radius },
    );

    for y in 0..h {
        for x in 0..w {
            let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
            // How far inside the box the pixel is, per axis.
            let (ox, oy) = ((px - edge_x) * sx, (py - edge_y) * sy);

            // Signed distance to the border, positive inside. Around the
            // corner that is the circle; past it, the nearer straight edge.
            let depth = if ox < radius && oy < radius {
                radius - (px - centre.0).hypot(py - centre.1)
            } else {
                ox.min(oy)
            };

            // The panel's own background is the whole cell with its outer
            // corner taken off by an arc of the same radius, so it meets the
            // square fill of the cells either side without a seam.
            let (bx, by) = ((px - outer.0) * sx, (py - outer.1) * sy);
            let filled = if bx < 0.0 && by < 0.0 {
                (0.5 + radius - (px - outer.0).hypot(py - outer.1)).clamp(0.0, 1.0)
            } else {
                1.0
            };
            if filled > 0.0 {
                canvas.blend(left + x, top + y, bg, (filled * 255.0) as u8);
            }

            let coverage = (stroke / 2.0 + 0.5 - depth.abs()).clamp(0.0, 1.0);
            if coverage > 0.0 {
                canvas.blend(left + x, top + y, colour, (coverage * 255.0) as u8);
            }
        }
    }
}

fn draw_arms(
    canvas: &mut Canvas,
    origin: (usize, usize),
    size: (usize, usize),
    arms: BoxArms,
    colour: Rgb,
) {
    let (left, top) = origin;
    let (w, h) = size;
    let light = (h / 10).max(1);
    let heavy = (light * 2).max(2);
    let stroke = |weight: u8| if weight >= 2 { heavy } else { light };

    // Arms meet on a centre line sized by the heaviest of them, so a light
    // arm joining a heavy one still lands on the same axis.
    let thickest = stroke(arms.weight());
    let cx = left + w.saturating_sub(thickest) / 2;
    let cy = top + h.saturating_sub(thickest) / 2;

    if arms.up > 0 {
        let t = stroke(arms.up);
        canvas.fill(cx, top, (t, cy + thickest - top), colour);
    }
    if arms.down > 0 {
        let t = stroke(arms.down);
        canvas.fill(cx, cy, (t, top + h - cy), colour);
    }
    if arms.left > 0 {
        let t = stroke(arms.left);
        canvas.fill(left, cy, (cx + thickest - left, t), colour);
    }
    if arms.right > 0 {
        let t = stroke(arms.right);
        canvas.fill(cx, cy, (left + w - cx, t), colour);
    }
}

fn draw_block(
    canvas: &mut Canvas,
    origin: (usize, usize),
    size: (usize, usize),
    block: BlockShape,
    colour: Rgb,
) {
    let (left, top) = origin;
    let (w, h) = size;

    match block {
        BlockShape::Full => canvas.fill(left, top, (w, h), colour),
        BlockShape::UpperHalf => canvas.fill(left, top, (w, h / 2), colour),
        BlockShape::LowerHalf => canvas.fill(left, top + h / 2, (w, h - h / 2), colour),
        BlockShape::LeftHalf => canvas.fill(left, top, (w / 2, h), colour),
        BlockShape::RightHalf => canvas.fill(left + w / 2, top, (w - w / 2, h), colour),
        BlockShape::LeftEighths(n) => {
            let width = (w * n as usize / 8).max(1);
            canvas.fill(left, top, (width, h), colour);
        }
        BlockShape::LowerEighths(n) => {
            let height = (h * n as usize / 8).max(1);
            canvas.fill(left, top + h - height, (w, height), colour);
        }
        BlockShape::Shade(alpha) => {
            for y in top..top + h {
                for x in left..left + w {
                    canvas.blend(x, y, colour, alpha);
                }
            }
        }
    }
}

/// Draws a cell's character, clipped to the cell.
///
/// A glyph's ink can be wider or taller than the cell it advances by, and
/// without a clip it spills onto the neighbour: over the end of a selected
/// row, past the edge of a panel, into the next column's background.
fn draw_glyph(
    cache: &mut GlyphCache,
    canvas: &mut Canvas,
    origin: (usize, usize),
    span: (usize, usize),
    baseline: usize,
    ch: char,
    cell: &SurfaceCell,
) {
    let style = GlyphStyle::of(
        cell.flags.contains(Flags::BOLD),
        cell.flags.contains(Flags::ITALIC),
    );
    let glyph = cache.glyph(ch, style);
    if glyph.width == 0 || glyph.height == 0 {
        return;
    }

    let x0 = origin.0 as i32 + glyph.left;
    let y0 = origin.1 as i32 + baseline as i32 - glyph.top;

    let (right, bottom) = ((origin.0 + span.0) as i32, (origin.1 + span.1) as i32);

    for gy in 0..glyph.height {
        let y = y0 + gy as i32;
        if y < origin.1 as i32 || y >= bottom {
            continue;
        }
        for gx in 0..glyph.width {
            let x = x0 + gx as i32;
            if x < origin.0 as i32 || x >= right {
                continue;
            }
            let alpha = glyph.coverage[gy * glyph.width + gx];
            if alpha > 0 {
                canvas.blend(x as usize, y as usize, cell.fg, alpha);
            }
        }
    }
}

/// Fills a cell's background with one or more corners rounded.
///
/// The radius is half the cell's height, which makes the end of a one-row run
/// a semicircle: a pill rather than a bar with the corners filed off. A cell
/// is narrower than that radius, so the left and right quadrants overlap and
/// each rounded corner has to be tested in turn; deciding which quadrant a
/// pixel is in first would put every pixel on the left.
fn fill_rounded(
    canvas: &mut Canvas,
    origin: (usize, usize),
    size: (usize, usize),
    cell: &SurfaceCell,
) {
    let (left, top) = origin;
    let (w, h) = (size.0 as f32, size.1 as f32);
    let radius = (h / 2.0).min(w);

    // Which half of the cell a corner owns is a property of the corner, not
    // of where its centre landed: a radius wider than half the cell puts the
    // right-hand centres left of the middle.
    let corners = [
        (Corners::TOP_LEFT, radius, radius, false, false),
        (Corners::TOP_RIGHT, w - radius, radius, true, false),
        (Corners::BOTTOM_RIGHT, w - radius, h - radius, true, true),
        (Corners::BOTTOM_LEFT, radius, h - radius, false, true),
    ];

    for y in 0..size.1 {
        for x in 0..size.0 {
            let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
            let mut coverage = 1.0f32;

            for (corner, cx, cy, right, lower) in corners {
                if !cell.round.contains(corner) {
                    continue;
                }
                // Only the arc's own quarter of the cell is cut away.
                let horizontal = if right { px > cx } else { px < cx };
                let vertical = if lower { py > cy } else { py < cy };
                if horizontal && vertical {
                    let distance = (px - cx).hypot(py - cy);
                    coverage = coverage.min((0.5 + radius - distance).clamp(0.0, 1.0));
                }
            }

            // Whatever the cell is laid on shows through the cut corner. It
            // is not always the window background: a selected row is laid on
            // a panel, and painting the window's colour there would punch a
            // hole in the panel.
            canvas.fill(left + x, top + y, (1, 1), cell.under);
            if coverage > 0.0 {
                canvas.blend(left + x, top + y, cell.bg, (coverage * 255.0) as u8);
            }
        }
    }
}
