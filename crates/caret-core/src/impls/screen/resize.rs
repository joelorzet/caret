#[allow(unused_imports)]
use crate::prelude::*;

impl Screen {
    pub fn resize(&mut self, cols: u16, rows: u16) {
        let cols = cols.max(1);
        let rows = rows.max(1);
        if cols == self.cols && rows == self.rows {
            return;
        }

        let old_rows = self.rows;
        // The room the pane has earned, not the room it is entitled to: a
        // resize is no reason to allocate a scrollback nobody has filled.
        let history_cap = self
            .cap
            .saturating_sub(self.rows as u32)
            .max(Screen::INITIAL_HISTORY.min(self.scrollback_limit))
            .min(self.scrollback_limit);
        let new_cap = rows as u32 + history_cap;
        let keep = self.len.min(new_cap);

        // Where the cursor is counted from the very top, which is the one
        // position that has to survive: everything above it is what has been
        // written, everything below it is blank room the last resize left.
        let cursor_row = self.history_len() + self.cursor.y as u32;

        // Shrinking drops rows, and it must drop the blank ones below the
        // cursor before it touches anything written above it. Keeping the
        // last `rows` blindly threw the content away and kept the padding.
        let wanted_end = (cursor_row + 1).max(rows as u32).min(self.len);
        let first_kept = wanted_end.saturating_sub(keep).min(self.len - keep);

        let w = cols as usize;
        let mut cells = vec![Cell::EMPTY; new_cap as usize * w].into_boxed_slice();
        let mut meta = vec![RowMeta(RowMeta::DIRTY); new_cap as usize].into_boxed_slice();
        let copy_w = w.min(self.cols as usize);

        for i in 0..keep {
            let src = self.slot(first_kept + i);
            let src_row = {
                let sw = self.cols as usize;
                &self.cells[src * sw..src * sw + copy_w]
            };
            let dst = i as usize;
            cells[dst * w..dst * w + copy_w].copy_from_slice(src_row);
            meta[dst] = RowMeta(self.meta[src].0 | RowMeta::DIRTY);
        }

        if copy_w < self.cols as usize {
            for m in meta.iter_mut() {
                m.0 &= !RowMeta::WRAPPED;
            }
        }

        let mut len = keep;
        if len < rows as u32 {
            len = rows as u32;
        }

        self.cells = cells;
        self.meta = meta;
        self.slots = (0..new_cap).collect::<Vec<_>>().into_boxed_slice();
        self.head = 0;
        self.len = len;
        self.cap = new_cap;
        self.cols = cols;
        self.rows = rows;

        // The cursor stays on the row it was on. Worked out from where that
        // row ended up rather than from how much the window changed by: a
        // difference in height says nothing about how much scrollback came
        // into view, and the two are not the same number.
        let moved = cursor_row.saturating_sub(first_kept);
        self.cursor.y = moved
            .saturating_sub(self.history_len())
            .min(rows as u32 - 1) as u16;
        let _ = old_rows;

        self.scroll_offset = self.scroll_offset.min(self.history_len());
        self.reset_region();
        self.clamp_cursor();
        self.saved_cursor.x = self.saved_cursor.x.min(cols - 1);
        self.saved_cursor.y = self.saved_cursor.y.min(rows - 1);
        self.all_dirty = true;
    }
}
