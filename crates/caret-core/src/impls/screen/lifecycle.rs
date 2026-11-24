#[allow(unused_imports)]
use crate::prelude::*;

impl Screen {
    /// How many rows of scrollback a screen starts out able to hold.
    ///
    /// The grid is one flat allocation of `cap * cols` cells, so sizing it
    /// for the whole configured scrollback at birth costs 32MB a pane at ten
    /// thousand lines and two hundred columns, written and resident, whether
    /// or not a single line is ever scrolled back to. It starts here instead
    /// and doubles as history fills, so a pane pays for the lines it has
    /// actually printed.
    pub(crate) const INITIAL_HISTORY: u32 = 512;

    pub fn new(cols: u16, rows: u16, scrollback: u32) -> Screen {
        let cols = cols.max(1);
        let rows = rows.max(1);
        let cap = rows as u32 + scrollback.min(Screen::INITIAL_HISTORY);
        let n = cap as usize * cols as usize;

        Screen {
            cols,
            rows,
            cap,
            cells: vec![Cell::EMPTY; n].into_boxed_slice(),
            meta: vec![RowMeta(RowMeta::DIRTY); cap as usize].into_boxed_slice(),
            slots: (0..cap).collect::<Vec<_>>().into_boxed_slice(),
            head: 0,
            len: rows as u32,
            scroll_offset: 0,
            scrollback_limit: scrollback,
            cursor: Cursor::default(),
            saved_cursor: Cursor::default(),
            pen: Cell::EMPTY,
            region: ScrollRegion {
                top: 0,
                bottom: rows - 1,
            },
            reflow: true,
            all_dirty: true,
            scratch: Vec::new(),
        }
    }

    /// The alternate screen: no scrollback, and no rewrapping, because the
    /// program drawing it redraws on every resize.
    pub fn alternate(cols: u16, rows: u16) -> Screen {
        Screen {
            reflow: false,
            ..Screen::new(cols, rows, 0)
        }
    }

    /// Doubles the ring, up to the configured scrollback.
    ///
    /// Called when history has filled what is allocated and there is still
    /// scrollback to earn. Doubling means a pane that does fill ten thousand
    /// lines reallocates five times over its life and then never again, and
    /// the copy is the rows it already has.
    pub(crate) fn grow(&mut self) {
        let ceiling = self.rows as u32 + self.scrollback_limit;
        if self.cap >= ceiling {
            return;
        }
        let cap = self.cap.saturating_mul(2).min(ceiling);

        let w = self.cols as usize;
        let mut cells = vec![Cell::EMPTY; cap as usize * w].into_boxed_slice();
        let mut meta = vec![RowMeta(RowMeta::DIRTY); cap as usize].into_boxed_slice();
        // Logical order, so the ring starts again from the top and `slots`
        // goes back to being the identity it was built as.
        for l in 0..self.len {
            let from = self.slot(l);
            let to = l as usize;
            cells[to * w..to * w + w].copy_from_slice(&self.cells[from * w..from * w + w]);
            meta[to] = self.meta[from];
        }

        self.cells = cells;
        self.meta = meta;
        self.slots = (0..cap).collect::<Vec<_>>().into_boxed_slice();
        self.head = 0;
        self.cap = cap;
    }

    pub fn memory_bytes(&self) -> usize {
        core::mem::size_of_val(&*self.cells)
            + core::mem::size_of_val(&*self.meta)
            + core::mem::size_of_val(&*self.slots)
            + self.scratch.capacity() * 4
    }

    pub fn reset(&mut self) {
        self.pen = Cell::EMPTY;
        self.cursor = Cursor::default();
        self.saved_cursor = Cursor::default();
        self.reset_region();
        self.erase_display(EraseMode::All);
        self.clear_history();
        self.all_dirty = true;
    }

    pub fn row_text(&self, l: u32) -> String {
        let row = self.logical_line(l);
        let end = row
            .iter()
            .rposition(|c| !c.is_blank() || c.bg != Color::DEFAULT)
            .map_or(0, |i| i + 1);
        row[..end]
            .iter()
            .filter(|c| !c.flags.contains(Flags::WIDE_TAIL))
            .map(|c| if c.ch == '\0' { ' ' } else { c.ch })
            .collect()
    }
}
