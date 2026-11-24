#[allow(unused_imports)]
use crate::prelude::*;

impl Screen {
    pub fn print(&mut self, ch: char, width: u16, autowrap: bool, insert: bool) {
        if width == 0 {
            return;
        }

        if self.cursor.pending_wrap && autowrap {
            let y = self.cursor.y;
            self.set_wrapped(y, true);
            self.cursor.x = 0;
            self.line_feed();
            self.cursor.pending_wrap = false;
        }

        if width == 2 && self.cursor.x + 1 >= self.cols {
            if autowrap {
                let y = self.cursor.y;
                self.set_wrapped(y, true);
                self.cursor.x = 0;
                self.line_feed();
            } else {
                self.cursor.x = self.cols - 2;
            }
        }

        if insert {
            self.insert_cells(width);
        }

        let (x, y) = (self.cursor.x, self.cursor.y);
        let pen = self.pen;
        let cols = self.cols;

        {
            let row = self.line_mut(y);
            row[x as usize] = Cell {
                ch,
                fg: pen.fg,
                bg: pen.bg,
                flags: pen.flags.intersection(Flags::SGR_MASK),
            };
            if width == 2 {
                row[x as usize].flags |= Flags::WIDE;
                if (x + 1) < cols {
                    row[x as usize + 1] =
                        Cell::wide_tail(pen.fg, pen.bg, pen.flags.intersection(Flags::SGR_MASK));
                }
            }
        }

        let next = x + width;
        if next >= cols {
            self.cursor.x = cols - 1;
            self.cursor.pending_wrap = true;
        } else {
            self.cursor.x = next;
            self.cursor.pending_wrap = false;
        }
    }
}
