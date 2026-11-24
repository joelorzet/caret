#[allow(unused_imports)]
use crate::prelude::*;

impl Screen {
    /// First logical row on display, so a mouse position can be turned into a
    /// [`Point`].
    pub fn display_row(&self, y: u16) -> u32 {
        self.display_base() + y.min(self.rows - 1) as u32
    }

    pub fn logical_wrapped(&self, row: u32) -> bool {
        if row >= self.len {
            return false;
        }
        self.meta[self.slot(row)].is_wrapped()
    }

    /// The selected text.
    ///
    /// Rows that wrapped because they filled the width are joined without a
    /// newline, so copying a long command line gives back one line rather
    /// than several. Trailing blanks are trimmed from each row.
    pub fn selection_text(&self, selection: &Selection) -> String {
        let (start, end) = selection.range();
        let last = self.len.saturating_sub(1);
        let (first_row, last_row) = (start.row.min(last), end.row.min(last));

        let mut out = String::new();
        for row in first_row..=last_row {
            let cells = self.logical_line(row);
            let from = if row == first_row {
                start.col as usize
            } else {
                0
            };
            let to = if row == last_row {
                (end.col as usize + 1).min(cells.len())
            } else {
                cells.len()
            };
            if from >= to {
                continue;
            }

            let slice = &cells[from..to];
            let trimmed = match slice.iter().rposition(|c| !c.is_blank()) {
                Some(i) => &slice[..=i],
                // A row of only blanks contributes nothing but its newline.
                None => &slice[..0],
            };
            for cell in trimmed {
                if !cell.flags.contains(Flags::WIDE_TAIL) {
                    out.push(if cell.ch == '\0' { ' ' } else { cell.ch });
                }
            }

            if row < last_row && !self.logical_wrapped(row) {
                out.push('\n');
            }
        }
        out
    }

    /// The word around `point`, for a double click.
    ///
    /// Returns an empty range when the cell itself is a separator, so
    /// double-clicking whitespace selects nothing rather than a whole line.
    pub fn word_at(&self, point: Point, separators: &str) -> Option<(Point, Point)> {
        let row = point.row.min(self.len.saturating_sub(1));
        let cells = self.logical_line(row);
        let col = point.col.min(self.cols.saturating_sub(1)) as usize;

        let is_separator =
            |cell: &Cell| cell.ch == '\0' || cell.ch == ' ' || separators.contains(cell.ch);
        if cells.get(col).is_none_or(is_separator) {
            return None;
        }

        let mut start = col;
        while start > 0 && !is_separator(&cells[start - 1]) {
            start -= 1;
        }
        let mut end = col;
        while end + 1 < cells.len() && !is_separator(&cells[end + 1]) {
            end += 1;
        }

        Some((Point::new(row, start as u16), Point::new(row, end as u16)))
    }

    /// The whole logical line around `point`, following wrapping in both
    /// directions, for a triple click.
    pub fn line_at(&self, point: Point) -> (Point, Point) {
        let last = self.len.saturating_sub(1);
        let row = point.row.min(last);

        let mut first = row;
        while first > 0 && self.logical_wrapped(first - 1) {
            first -= 1;
        }
        let mut final_row = row;
        while final_row < last && self.logical_wrapped(final_row) {
            final_row += 1;
        }

        (
            Point::new(first, 0),
            Point::new(final_row, self.cols.saturating_sub(1)),
        )
    }
}
