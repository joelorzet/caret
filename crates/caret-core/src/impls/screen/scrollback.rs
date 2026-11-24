#[allow(unused_imports)]
use crate::prelude::*;

impl Screen {
    #[inline]
    pub fn scroll_offset(&self) -> u32 {
        self.scroll_offset
    }

    pub fn scroll_display(&mut self, delta: i32) -> bool {
        let max = self.history_len();
        let next = (self.scroll_offset as i64 + delta as i64).clamp(0, max as i64) as u32;
        if next != self.scroll_offset {
            self.scroll_offset = next;
            self.all_dirty = true;
            true
        } else {
            false
        }
    }

    pub fn scroll_to_bottom(&mut self) -> bool {
        self.scroll_display(-(self.scroll_offset as i32))
    }

    pub fn clear_history(&mut self) {
        let keep = self.rows as u32;
        let drop = self.len - keep;
        if drop == 0 {
            self.scroll_offset = 0;
            return;
        }
        self.head = ((self.head as u64 + drop as u64) % self.cap as u64) as u32;
        self.len = keep;
        self.scroll_offset = 0;
        self.all_dirty = true;
    }
}
