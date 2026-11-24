#[allow(unused_imports)]
use crate::prelude::*;

impl Screen {
    pub fn resize(&mut self, cols: u16, rows: u16) {
        let cols = cols.max(1);
        let rows = rows.max(1);
        if cols == self.cols && rows == self.rows {
            return;
        }
        // A different width means every wrapped line is wrapped in the
        // wrong place, and only laying them out again puts that right.
        if cols != self.cols && self.reflow {
            self.rewrap(cols, rows);
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

        // Where the cursor is counted from the very top, which is the one
        // position that has to survive: everything above it is what has been
        // written, everything below it is blank room the last resize left.
        let cursor_row = self.history_len() + self.cursor.y as u32;

        // How much of the buffer is content. Every resize pads the length
        // out to the new height, and that padding is not something anybody
        // typed: counted as content, it becomes history the next time the
        // window grows, and the prompt ends up stranded in the middle of the
        // pane with blank rows above it.
        let written = self.written_end(cursor_row);
        let keep = written.min(new_cap);

        // Shrinking drops rows, and it must drop the blank ones below the
        // cursor before it touches anything written above it. Keeping the
        // last `rows` blindly threw the content away and kept the padding.
        let wanted_end = (cursor_row + 1).max(rows as u32).min(written);
        let first_kept = wanted_end.saturating_sub(keep).min(written - keep);

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

    /// Lays the content out again at a new width.
    ///
    /// A line that wrapped is one line: it is taken back apart, re-broken
    /// at the new width, and written back. What has to survive is the
    /// character the cursor is on, not the row it was on, because the row
    /// is about to be a different row.
    fn rewrap(&mut self, cols: u16, rows: u16) {
        let cursor_row = self.history_len() + self.cursor.y as u32;
        let Taken {
            mut lines,
            starts,
            cursor_at,
        } = self.take_lines(cursor_row);

        // The cursor may sit past the end of its line, which is where a
        // prompt leaves it. It needs somewhere to be.
        if let Some((line, offset)) = cursor_at {
            let blank = self.erase_cell();
            if lines[line].len() < offset {
                lines[line].resize(offset, blank);
            }
        }

        let history_cap = self
            .cap
            .saturating_sub(self.rows as u32)
            .max(Screen::INITIAL_HISTORY.min(self.scrollback_limit))
            .min(self.scrollback_limit);
        let new_cap = rows as u32 + history_cap;

        let Rewrapped {
            mut rows_out,
            mut cursor,
            mut line_rows,
        } = lay_out(&lines, cols, cursor_at);

        // Older rows go first when there is more than there is room for.
        if rows_out.len() as u32 > new_cap {
            let drop = rows_out.len() - new_cap as usize;
            rows_out.drain(..drop);
            if let Some(spot) = cursor.as_mut() {
                spot.row = spot.row.saturating_sub(drop);
            }
            for row in line_rows.iter_mut() {
                *row = row.saturating_sub(drop);
            }
        }

        let w = cols as usize;
        let mut cells = vec![Cell::EMPTY; new_cap as usize * w].into_boxed_slice();
        let mut meta = vec![RowMeta(RowMeta::DIRTY); new_cap as usize].into_boxed_slice();
        for (index, row) in rows_out.iter().enumerate() {
            cells[index * w..index * w + row.cells.len()].copy_from_slice(&row.cells);
            if row.continues {
                meta[index].0 |= RowMeta::WRAPPED;
            }
        }

        // Rebuilding the buffer renumbers every row, so the marks are
        // renumbered with it: each one moves to the line it was on.
        self.prompts = self
            .prompts
            .iter()
            .filter_map(|marked| marked.checked_sub(self.dropped))
            .filter_map(|old| {
                let line = starts.partition_point(|start| *start as u64 <= old);
                line_rows.get(line.checked_sub(1)?).map(|row| *row as u64)
            })
            .collect();
        self.prompts.dedup();
        self.dropped = 0;

        self.cells = cells;
        self.meta = meta;
        self.slots = (0..new_cap).collect::<Vec<_>>().into_boxed_slice();
        self.head = 0;
        self.len = (rows_out.len() as u32).max(rows as u32);
        self.cap = new_cap;
        self.cols = cols;
        self.rows = rows;

        let spot = cursor.unwrap_or(Spot {
            row: rows_out.len().saturating_sub(1),
            col: 0,
        });
        self.cursor.y = (spot.row as u32)
            .saturating_sub(self.history_len())
            .min(rows as u32 - 1) as u16;
        self.cursor.x = spot.col.min(cols - 1);

        self.scroll_offset = self.scroll_offset.min(self.history_len());
        self.reset_region();
        self.clamp_cursor();
        self.saved_cursor.x = self.saved_cursor.x.min(cols - 1);
        self.saved_cursor.y = self.saved_cursor.y.min(rows - 1);
        self.all_dirty = true;
    }

    /// The content as logical lines, with where the cursor sits in them.
    ///
    /// Trailing blanks go: they are the width the line was written at, not
    /// something anybody typed, and keeping them would wrap a short line
    /// onto rows of nothing.
    fn take_lines(&self, cursor_row: u32) -> Taken {
        let written = self.written_end(cursor_row);
        let mut lines: Vec<Vec<Cell>> = Vec::new();
        // The row each line started on, so a mark can move with its line.
        let mut starts: Vec<u32> = Vec::new();
        let mut cursor_at = None;
        let mut row = 0;

        while row < written {
            starts.push(row);
            let mut cells: Vec<Cell> = Vec::new();
            let mut last = row;
            loop {
                if last == cursor_row {
                    cursor_at = Some((lines.len(), cells.len() + self.cursor.x as usize));
                }
                cells.extend_from_slice(self.cells_of(self.slot(last)));
                if self.meta[self.slot(last)].is_wrapped() && last + 1 < written {
                    last += 1;
                } else {
                    break;
                }
            }
            while cells.last().is_some_and(is_padding) {
                cells.pop();
            }
            lines.push(cells);
            row = last + 1;
        }

        if lines.is_empty() {
            lines.push(Vec::new());
            starts.push(0);
        }
        Taken {
            lines,
            starts,
            cursor_at,
        }
    }

    /// One past the last row holding anything, never above the cursor.
    ///
    /// Blank rows below the cursor are room, not content: either padding a
    /// previous resize added to fill the screen, or space a program has not
    /// drawn into yet. A program that is using the lower half of the screen
    /// has not left it blank, so nothing it drew is trimmed.
    fn written_end(&self, cursor_row: u32) -> u32 {
        let floor = (cursor_row + 1).min(self.len);
        let mut end = self.len;
        while end > floor && self.row_is_blank(end - 1) {
            end -= 1;
        }
        end
    }

    fn row_is_blank(&self, l: u32) -> bool {
        self.logical_line(l).iter().all(is_padding)
    }
}

/// A cell that holds nothing anybody chose: blank, and not painted.
fn is_padding(cell: &Cell) -> bool {
    cell.is_blank() && cell.bg == Color::DEFAULT
}

/// The content pulled back apart into the lines it was written as.
struct Taken {
    lines: Vec<Vec<Cell>>,
    /// The row each line started on, so a mark can move with its line.
    starts: Vec<u32>,
    /// The line the cursor is on and how far along it, if it is here.
    cursor_at: Option<(usize, usize)>,
}

/// A row the layout produced, and whether its line goes on below it.
struct Row {
    cells: Vec<Cell>,
    continues: bool,
}

/// Where the cursor ended up once the lines were laid out again.
struct Spot {
    row: usize,
    col: u16,
}

struct Rewrapped {
    rows_out: Vec<Row>,
    cursor: Option<Spot>,
    /// The row each line now starts on.
    line_rows: Vec<usize>,
}

/// Breaks lines into rows of `cols`, saying which rows continue.
///
/// A wide character is two cells and cannot be split, so a row ends early
/// rather than leaving half a glyph at each end.
fn lay_out(lines: &[Vec<Cell>], cols: u16, cursor_at: Option<(usize, usize)>) -> Rewrapped {
    let width = cols as usize;
    let mut rows: Vec<Row> = Vec::new();
    let mut cursor = None;
    let mut line_rows: Vec<usize> = Vec::new();

    for (index, line) in lines.iter().enumerate() {
        line_rows.push(rows.len());
        let mut start = 0;
        loop {
            let mut end = (start + width).min(line.len());
            if end < line.len() && line[end].flags.contains(Flags::WIDE_TAIL) {
                end -= 1;
            }
            if let Some((cursor_line, offset)) = cursor_at {
                if cursor_line == index && offset >= start && (offset < end || end == line.len()) {
                    cursor = Some(Spot {
                        row: rows.len(),
                        col: (offset - start).min(width - 1) as u16,
                    });
                }
            }
            rows.push(Row {
                cells: line[start..end].to_vec(),
                continues: end < line.len(),
            });
            start = end;
            if start >= line.len() {
                break;
            }
        }
    }
    Rewrapped {
        rows_out: rows,
        cursor,
        line_rows,
    }
}
