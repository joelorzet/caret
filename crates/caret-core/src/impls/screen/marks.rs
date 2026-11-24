#[allow(unused_imports)]
use crate::prelude::*;

/// How many prompts to remember. Each is eight bytes, and a pane that has
/// run two thousand commands has scrolled the first ones out of reach
/// anyway.
const REMEMBERED: usize = 2_048;

impl Screen {
    /// Records that a command's prompt begins on the cursor's row.
    ///
    /// The shell says so; without it there is no way to tell a prompt from
    /// any other line, since a prompt is whatever the shell drew.
    pub fn mark_prompt(&mut self) {
        let row = self.absolute(self.history_len() + self.cursor.y as u32);
        if self.prompts.last() == Some(&row) {
            return;
        }
        self.prompts.push(row);
        if self.prompts.len() > REMEMBERED {
            let over = self.prompts.len() - REMEMBERED;
            self.prompts.drain(..over);
        }
    }

    /// The prompt before `row`, as a row in the buffer now.
    pub fn prompt_above(&self, row: u32) -> Option<u32> {
        let from = self.absolute(row);
        self.prompts
            .iter()
            .rev()
            .find(|marked| **marked < from)
            .and_then(|marked| self.present(*marked))
    }

    /// The prompt after `row`, as a row in the buffer now.
    pub fn prompt_below(&self, row: u32) -> Option<u32> {
        let from = self.absolute(row);
        self.prompts
            .iter()
            .find(|marked| **marked > from)
            .and_then(|marked| self.present(*marked))
    }

    pub fn prompt_count(&self) -> usize {
        self.prompts.len()
    }

    /// A row's number counted from the first the pane ever had.
    fn absolute(&self, row: u32) -> u64 {
        self.dropped + row as u64
    }

    /// Where an absolute row is now, if it is still here.
    fn present(&self, absolute: u64) -> Option<u32> {
        let row = absolute.checked_sub(self.dropped)?;
        (row < self.len as u64).then_some(row as u32)
    }

    /// Forgets prompts on rows that have left the buffer.
    pub(crate) fn forget_dropped_prompts(&mut self) {
        let floor = self.dropped;
        self.prompts.retain(|marked| *marked >= floor);
    }
}
