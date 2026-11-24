#[allow(unused_imports)]
use crate::prelude::*;

impl Surface {
    pub fn new(cols: u16, rows: u16, bg: Rgb) -> Surface {
        let blank = SurfaceCell {
            ch: ' ',
            fg: bg,
            bg,
            flags: Flags::NONE,
            round: Corners::NONE,
            under: bg,
        };
        Surface {
            cols,
            rows,
            cells: vec![blank; cols as usize * rows as usize],
        }
    }

    pub fn put(&mut self, x: u16, y: u16, ch: char, style: Style) {
        self.set(
            x,
            y,
            SurfaceCell {
                ch,
                fg: style.fg,
                bg: style.bg,
                flags: style.flags,
                round: Corners::NONE,
                under: style.bg,
            },
        );
    }

    pub fn size(&self) -> (u16, u16) {
        (self.cols, self.rows)
    }

    pub fn get(&self, x: u16, y: u16) -> Option<SurfaceCell> {
        if x >= self.cols || y >= self.rows {
            return None;
        }
        Some(self.cells[y as usize * self.cols as usize + x as usize])
    }

    pub fn set(&mut self, x: u16, y: u16, cell: SurfaceCell) {
        if x < self.cols && y < self.rows {
            self.cells[y as usize * self.cols as usize + x as usize] = cell;
        }
    }

    /// Fills a rectangle whose ends are rounded, for a selection bar or a
    /// chip. The rounding lives on the cells at the ends; everything between
    /// them is an ordinary fill.
    pub fn fill_rounded(&mut self, rect: Rect, bg: Rgb, under: Rgb) {
        self.fill(rect, bg);
        if rect.w == 0 || rect.h == 0 {
            return;
        }
        let last = rect.right() - 1;
        for y in rect.y..rect.bottom().min(self.rows) {
            let top = y == rect.y;
            let bottom = y + 1 == rect.bottom();
            let left = match (top, bottom) {
                (true, true) => Corners::LEFT,
                (true, false) => Corners::TOP_LEFT,
                (false, true) => Corners::BOTTOM_LEFT,
                (false, false) => Corners::NONE,
            };
            self.round(rect.x, y, left, under);
            self.round(
                last,
                y,
                match (top, bottom) {
                    (true, true) => Corners::RIGHT,
                    (true, false) => Corners::TOP_RIGHT,
                    (false, true) => Corners::BOTTOM_RIGHT,
                    (false, false) => Corners::NONE,
                },
                under,
            );
        }
    }

    /// Rounds a rectangle's corners without disturbing what is in it.
    ///
    /// For content that is drawn by something else, such as a terminal's own
    /// grid, and only needs its corners taken off.
    pub fn round_corners(&mut self, rect: Rect, under: Rgb) {
        if rect.w == 0 || rect.h == 0 {
            return;
        }
        let last_x = rect.right() - 1;
        let last_y = rect.bottom() - 1;
        let left = if rect.h == 1 {
            Corners::LEFT
        } else {
            Corners::TOP_LEFT
        };
        let right = if rect.h == 1 {
            Corners::RIGHT
        } else {
            Corners::TOP_RIGHT
        };
        self.round(rect.x, rect.y, left, under);
        self.round(last_x, rect.y, right, under);
        if rect.h > 1 {
            self.round(rect.x, last_y, Corners::BOTTOM_LEFT, under);
            self.round(last_x, last_y, Corners::BOTTOM_RIGHT, under);
        }
    }

    fn round(&mut self, x: u16, y: u16, corners: Corners, under: Rgb) {
        if corners.is_empty() {
            return;
        }
        if let Some(mut cell) = self.get(x, y) {
            cell.round = cell.round.union(corners);
            cell.under = under;
            self.set(x, y, cell);
        }
    }

    pub fn fill(&mut self, rect: Rect, bg: Rgb) {
        let blank = SurfaceCell {
            ch: ' ',
            fg: bg,
            bg,
            flags: Flags::NONE,
            round: Corners::NONE,
            under: bg,
        };
        for y in rect.y..rect.bottom().min(self.rows) {
            for x in rect.x..rect.right().min(self.cols) {
                self.set(x, y, blank);
            }
        }
    }

    /// Writes `text` clipped to `width` columns.
    pub fn text(&mut self, x: u16, y: u16, width: u16, text: &str, style: Style) {
        for (offset, ch) in text.chars().enumerate() {
            let at = x.saturating_add(offset as u16);
            if offset as u16 >= width || at >= self.cols {
                break;
            }
            self.put(at, y, ch, style);
        }
    }

    /// Writes `text` over cells that are already there, keeping whatever
    /// rounding they carry.
    ///
    /// Rounding belongs to the shape underneath, not to the glyph on top, so
    /// labelling a pill must not square its ends off again.
    pub fn text_over(&mut self, x: u16, y: u16, width: u16, text: &str, style: Style) {
        for (offset, ch) in text.chars().enumerate() {
            let at = x.saturating_add(offset as u16);
            if offset as u16 >= width || at >= self.cols {
                break;
            }
            let Some(beneath) = self.get(at, y) else {
                break;
            };
            self.set(
                at,
                y,
                SurfaceCell {
                    ch,
                    fg: style.fg,
                    bg: style.bg,
                    flags: style.flags,
                    round: beneath.round,
                    under: beneath.under,
                },
            );
        }
    }

    pub fn hline(&mut self, rect: Rect, ch: char, fg: Rgb, bg: Rgb) {
        for x in rect.x..rect.right() {
            for y in rect.y..rect.bottom() {
                self.set(
                    x,
                    y,
                    SurfaceCell {
                        ch,
                        fg,
                        bg,
                        flags: Flags::NONE,
                        round: Corners::NONE,
                        under: bg,
                    },
                );
            }
        }
    }

    /// Emits the frame as one escape-sequence string.
    ///
    /// Colour and attribute changes are only written when they actually
    /// change, which keeps a full redraw to a few kilobytes.
    pub fn render(&self) -> String {
        let mut out = String::with_capacity(self.cells.len() * 4);
        for y in 0..self.rows {
            out.push_str(&format!("\x1b[{};1H", y + 1));
            // Styles are tracked per row: the cursor jump between rows means
            // the previous row's trailing style cannot be relied on.
            let mut last: Option<(Rgb, Rgb, Flags)> = None;
            let mut x = 0;
            while x < self.cols {
                let cell = self.cells[y as usize * self.cols as usize + x as usize];

                // The host terminal advances two columns for a wide glyph, so
                // the trailing half must not be emitted.
                if cell.flags.contains(Flags::WIDE_TAIL) {
                    x += 1;
                    continue;
                }

                let style = (cell.fg, cell.bg, cell.flags);
                if last != Some(style) {
                    out.push_str("\x1b[0m");
                    if cell.flags.contains(Flags::BOLD) {
                        out.push_str("\x1b[1m");
                    }
                    if cell.flags.contains(Flags::ITALIC) {
                        out.push_str("\x1b[3m");
                    }
                    if cell.flags.any_underline() {
                        out.push_str("\x1b[4m");
                    }
                    if cell.flags.contains(Flags::STRIKE) {
                        out.push_str("\x1b[9m");
                    }
                    out.push_str(&format!(
                        "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m",
                        cell.fg.r, cell.fg.g, cell.fg.b, cell.bg.r, cell.bg.g, cell.bg.b
                    ));
                    last = Some(style);
                }
                out.push(if cell.ch == '\0' { ' ' } else { cell.ch });
                x += 1;
            }
        }

        out.push_str("\x1b[0m");
        out
    }
}
