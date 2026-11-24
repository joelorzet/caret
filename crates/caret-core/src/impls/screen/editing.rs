#[allow(unused_imports)]
use crate::prelude::*;

impl Screen {
    pub fn insert_cells(&mut self, n: u16) {
        let (x, y) = (self.cursor.x, self.cursor.y);
        let cols = self.cols;
        let n = n.min(cols - x);
        let blank = self.erase_cell();
        let row = self.line_mut(y);
        row[x as usize..].rotate_right(n as usize);
        for c in &mut row[x as usize..(x + n) as usize] {
            *c = blank;
        }
    }

    pub fn delete_cells(&mut self, n: u16) {
        let (x, y) = (self.cursor.x, self.cursor.y);
        let cols = self.cols;
        let n = n.min(cols - x);
        let blank = self.erase_cell();
        let row = self.line_mut(y);
        row[x as usize..].rotate_left(n as usize);
        for c in &mut row[(cols - n) as usize..] {
            *c = blank;
        }
    }

    pub fn erase_cells(&mut self, n: u16) {
        let (x, y) = (self.cursor.x, self.cursor.y);
        let end = (x + n).min(self.cols);
        let blank = self.erase_cell();
        let row = self.line_mut(y);
        for c in &mut row[x as usize..end as usize] {
            *c = blank;
        }
    }

    pub fn insert_lines(&mut self, n: u16) {
        let r = self.region;
        if self.cursor.y < r.top || self.cursor.y > r.bottom {
            return;
        }
        let top = self.cursor.y;
        self.scroll_down_in(top, r.bottom, n);
        self.cursor.x = 0;
        self.cursor.pending_wrap = false;
    }

    pub fn delete_lines(&mut self, n: u16) {
        let r = self.region;
        if self.cursor.y < r.top || self.cursor.y > r.bottom {
            return;
        }
        let top = self.cursor.y;
        self.scroll_up_in(top, r.bottom, n);
        self.cursor.x = 0;
        self.cursor.pending_wrap = false;
    }

    pub fn erase_line(&mut self, mode: EraseMode) {
        let (x, y) = (self.cursor.x, self.cursor.y);
        let cols = self.cols as usize;
        let blank = self.erase_cell();
        let row = self.line_mut(y);
        let range = match mode {
            EraseMode::ToEnd => x as usize..cols,
            EraseMode::ToStart => 0..(x as usize + 1).min(cols),
            EraseMode::All => 0..cols,
        };
        for c in &mut row[range] {
            *c = blank;
        }
        self.set_wrapped(y, false);
    }

    pub fn erase_display(&mut self, mode: EraseMode) {
        let (x, y) = (self.cursor.x, self.cursor.y);
        let blank = self.erase_cell();
        let (rows, cols) = (self.rows, self.cols as usize);

        let full_rows = match mode {
            EraseMode::ToEnd => {
                let row = self.line_mut(y);
                for c in &mut row[x as usize..cols] {
                    *c = blank;
                }
                (y + 1)..rows
            }
            EraseMode::ToStart => {
                let row = self.line_mut(y);
                for c in &mut row[0..(x as usize + 1).min(cols)] {
                    *c = blank;
                }
                0..y
            }
            EraseMode::All => 0..rows,
        };

        for yy in full_rows {
            let row = self.line_mut(yy);
            row.fill(blank);
            self.set_wrapped(yy, false);
        }
        self.all_dirty = true;
    }

    pub fn erase_saved_lines(&mut self) {
        self.clear_history();
    }
}
