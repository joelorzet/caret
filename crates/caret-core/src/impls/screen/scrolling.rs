#[allow(unused_imports)]
use crate::prelude::*;

impl Screen {
    pub(crate) fn push_row(&mut self) {
        // History fills the allocation before the allocation is enlarged, so
        // the common case is a pane that never reaches its configured
        // scrollback and never pays for it.
        if self.len == self.cap {
            self.grow();
        }
        if self.len < self.cap {
            self.len += 1;
        } else {
            // Dropping logical row 0 makes its slot the new bottom row, so a
            // full scrollback recycles in place without reallocating.
            self.head = ((self.head as u64 + 1) % self.cap as u64) as u32;
        }

        let slot = self.slot(self.len - 1);
        let blank = self.erase_cell();
        self.cells_of_mut(slot).fill(blank);
        self.meta[slot] = RowMeta(RowMeta::DIRTY);

        // Keep a scrolled-back view on the content the user is reading
        // instead of letting new output drag it along.
        if self.scroll_offset > 0 {
            self.scroll_offset = (self.scroll_offset + 1).min(self.history_len());
        }
        self.all_dirty = true;
    }

    pub fn scroll_up(&mut self, n: u16) {
        let r = self.region;
        self.scroll_up_in(r.top, r.bottom, n);
    }

    pub(crate) fn scroll_up_in(&mut self, top: u16, bottom: u16, n: u16) {
        if n == 0 || top >= self.rows {
            return;
        }
        let bottom = bottom.min(self.rows - 1);
        let height = bottom - top + 1;
        let n = n.min(height);

        // A full-screen scroll retires rows into scrollback, which the ring
        // does in O(1). A partial region must discard them, as on a VT.
        if top == 0 && bottom == self.rows - 1 && self.scrollback_limit > 0 {
            for _ in 0..n {
                self.push_row();
            }
            return;
        }

        self.rotate_region(top, height, n, true);
    }

    pub fn scroll_down(&mut self, n: u16) {
        let r = self.region;
        self.scroll_down_in(r.top, r.bottom, n);
    }

    pub(crate) fn scroll_down_in(&mut self, top: u16, bottom: u16, n: u16) {
        if n == 0 || top >= self.rows {
            return;
        }
        let bottom = bottom.min(self.rows - 1);
        let height = bottom - top + 1;
        let n = n.min(height);
        self.rotate_region(top, height, n, false);
    }

    pub(crate) fn rotate_region(&mut self, top: u16, height: u16, n: u16, up: bool) {
        let base = self.vp_base() + top as u32;
        let height = height as u32;
        let n = n as u32;

        self.scratch.clear();
        self.scratch.reserve(n as usize);

        if up {
            for i in 0..n {
                self.scratch.push(self.slot(base + i) as u32);
            }
            for i in 0..height - n {
                let v = self.slot(base + i + n) as u32;
                self.set_slot(base + i, v);
            }
            for i in 0..n {
                let v = self.scratch[i as usize];
                self.set_slot(base + height - n + i, v);
            }
        } else {
            for i in 0..n {
                self.scratch.push(self.slot(base + height - n + i) as u32);
            }
            for i in (0..height - n).rev() {
                let v = self.slot(base + i) as u32;
                self.set_slot(base + i + n, v);
            }
            for i in 0..n {
                let v = self.scratch[i as usize];
                self.set_slot(base + i, v);
            }
        }

        let blank = self.erase_cell();
        let exposed_start = if up { height - n } else { 0 };
        for i in 0..n {
            let slot = self.slot(base + exposed_start + i);
            self.cells_of_mut(slot).fill(blank);
            self.meta[slot] = RowMeta(RowMeta::DIRTY);
        }
        self.all_dirty = true;
    }
}
