#[allow(unused_imports)]
use crate::prelude::*;

impl Screen {
    #[inline]
    pub fn cursor(&self) -> Cursor {
        self.cursor
    }

    #[inline]
    pub fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursor
    }

    #[inline]
    pub fn pen(&self) -> Cell {
        self.pen
    }

    pub fn set_pen_fg(&mut self, fg: Color) {
        self.pen.fg = fg;
    }

    pub fn set_pen_bg(&mut self, bg: Color) {
        self.pen.bg = bg;
    }

    pub fn pen_flags_mut(&mut self) -> &mut Flags {
        &mut self.pen.flags
    }

    pub fn reset_pen(&mut self) {
        self.pen = Cell::EMPTY;
    }

    #[inline]
    pub(crate) fn erase_cell(&self) -> Cell {
        Cell::blank(self.pen.bg)
    }

    pub fn save_cursor(&mut self) {
        self.saved_cursor = self.cursor;
    }

    pub fn restore_cursor(&mut self) {
        self.cursor = self.saved_cursor;
        self.clamp_cursor();
    }

    pub(crate) fn clamp_cursor(&mut self) {
        self.cursor.x = self.cursor.x.min(self.cols - 1);
        self.cursor.y = self.cursor.y.min(self.rows - 1);
    }
}
