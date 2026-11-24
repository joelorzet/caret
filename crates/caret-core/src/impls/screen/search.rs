#[allow(unused_imports)]
use crate::prelude::*;

impl Screen {
    /// Every occurrence of `needle`, oldest first, as inclusive start and end
    /// points.
    ///
    /// Searching logical rows covers the whole scrollback rather than what is
    /// on screen, and a line that wrapped is searched as the one line it is:
    /// a phrase broken by the right-hand edge is still a phrase.
    pub fn search_all(&self, needle: &str, case_sensitive: bool) -> Vec<(Point, Point)> {
        let pattern = fold(needle, case_sensitive);
        if pattern.is_empty() {
            return Vec::new();
        }

        let mut out = Vec::new();
        let mut line: Vec<(u32, u16, char)> = Vec::new();
        let mut row = 0;

        while row < self.len {
            line.clear();
            let mut last = row;
            loop {
                self.collect_row(last, case_sensitive, &mut line);
                if self.wrapped(last) && last + 1 < self.len {
                    last += 1;
                } else {
                    break;
                }
            }

            if line.len() >= pattern.len() {
                for start in 0..=line.len() - pattern.len() {
                    let hit = pattern
                        .iter()
                        .enumerate()
                        .all(|(offset, want)| line[start + offset].2 == *want);
                    if hit {
                        let (first_row, first_col, _) = line[start];
                        let (last_row, last_col, _) = line[start + pattern.len() - 1];
                        out.push((
                            Point::new(first_row, first_col),
                            Point::new(last_row, last_col),
                        ));
                    }
                }
            }
            row = last + 1;
        }
        out
    }

    /// The characters of one stored row, with where each one is.
    fn collect_row(&self, row: u32, case_sensitive: bool, into: &mut Vec<(u32, u16, char)>) {
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
            into.push((row, column as u16, ch));
        }
    }

    /// Whether a stored row continues onto the next one.
    fn wrapped(&self, row: u32) -> bool {
        self.meta[self.slot(row.min(self.len.saturating_sub(1)))].is_wrapped()
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
