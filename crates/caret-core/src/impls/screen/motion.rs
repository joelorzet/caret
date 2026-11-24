#[allow(unused_imports)]
use crate::prelude::*;

impl Screen {
    pub fn move_to(&mut self, x: u16, y: u16) {
        self.cursor.x = x.min(self.cols - 1);
        self.cursor.y = y.min(self.rows - 1);
        self.cursor.pending_wrap = false;
    }

    pub fn move_x(&mut self, x: u16) {
        self.cursor.x = x.min(self.cols - 1);
        self.cursor.pending_wrap = false;
    }

    pub fn move_y(&mut self, y: u16) {
        self.cursor.y = y.min(self.rows - 1);
        self.cursor.pending_wrap = false;
    }

    pub fn move_right(&mut self, n: u16) {
        self.cursor.x = self.cursor.x.saturating_add(n).min(self.cols - 1);
        self.cursor.pending_wrap = false;
    }

    pub fn move_left(&mut self, n: u16) {
        self.cursor.x = self.cursor.x.saturating_sub(n);
        self.cursor.pending_wrap = false;
    }

    pub fn move_up(&mut self, n: u16) {
        let floor = if self.cursor.y >= self.region.top {
            self.region.top
        } else {
            0
        };
        self.cursor.y = self.cursor.y.saturating_sub(n).max(floor);
        self.cursor.pending_wrap = false;
    }

    pub fn move_down(&mut self, n: u16) {
        let ceil = if self.cursor.y <= self.region.bottom {
            self.region.bottom
        } else {
            self.rows - 1
        };
        self.cursor.y = self.cursor.y.saturating_add(n).min(ceil);
        self.cursor.pending_wrap = false;
    }

    pub fn carriage_return(&mut self) {
        self.cursor.x = 0;
        self.cursor.pending_wrap = false;
    }

    pub fn line_feed(&mut self) {
        self.cursor.pending_wrap = false;
        if self.cursor.y == self.region.bottom {
            self.scroll_up(1);
        } else if self.cursor.y < self.rows - 1 {
            self.cursor.y += 1;
        }
    }

    pub fn reverse_line_feed(&mut self) {
        self.cursor.pending_wrap = false;
        if self.cursor.y == self.region.top {
            self.scroll_down(1);
        } else {
            self.cursor.y = self.cursor.y.saturating_sub(1);
        }
    }
}
