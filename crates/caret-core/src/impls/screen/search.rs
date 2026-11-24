#[allow(unused_imports)]
use crate::prelude::*;

impl Screen {
    /// Every occurrence of `needle`, oldest first, as inclusive start and end
    /// points.
    ///
    /// Searching logical rows means the whole scrollback is covered, not just
    /// what is on screen. A match is confined to one row: a line that wrapped
    /// is stored as separate rows, so an occurrence straddling the wrap is not
    /// found. See `docs/limitations.md`.
    pub fn search_all(&self, needle: &str, case_sensitive: bool) -> Vec<(Point, Point)> {
        let pattern = fold(needle, case_sensitive);
        if pattern.is_empty() {
            return Vec::new();
        }

        let mut out = Vec::new();
        let mut row_chars: Vec<(u16, char)> = Vec::new();

        for row in 0..self.len {
            row_chars.clear();
            for (column, cell) in self.logical_line(row).iter().enumerate() {
                // The trailing half of a wide glyph is not its own character.
                if cell.flags.contains(Flags::WIDE_TAIL) {
                    continue;
                }
                let ch = if cell.ch == '\0' { ' ' } else { cell.ch };
                let ch = if case_sensitive {
                    ch
                } else {
                    ch.to_lowercase().next().unwrap_or(ch)
                };
                row_chars.push((column as u16, ch));
            }

            if row_chars.len() < pattern.len() {
                continue;
            }
            for start in 0..=row_chars.len() - pattern.len() {
                let hit = pattern
                    .iter()
                    .enumerate()
                    .all(|(offset, want)| row_chars[start + offset].1 == *want);
                if hit {
                    let end = start + pattern.len() - 1;
                    out.push((
                        Point::new(row, row_chars[start].0),
                        Point::new(row, row_chars[end].0),
                    ));
                }
            }
        }
        out
    }

    /// Brings `row` into view, keeping it a third of the way down so there is
    /// context above and below.
    pub fn scroll_to_row(&mut self, row: u32) {
        let viewport_base = self.len - self.rows as u32;
        let target = row.saturating_sub(self.rows as u32 / 3);
        let offset = viewport_base.saturating_sub(target);
        self.scroll_offset = offset.min(self.history_len());
        self.all_dirty = true;
    }
}

fn fold(needle: &str, case_sensitive: bool) -> Vec<char> {
    if case_sensitive {
        needle.chars().collect()
    } else {
        needle.to_lowercase().chars().collect()
    }
}
