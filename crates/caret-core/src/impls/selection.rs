#[allow(unused_imports)]
use crate::prelude::*;

impl Selection {
    pub fn new(anchor: Point, mode: SelectionMode) -> Selection {
        Selection {
            anchor,
            head: anchor,
            mode,
        }
    }

    /// A selection spanning two points, for a word or line picked out in one
    /// gesture.
    pub fn spanning(start: Point, end: Point, mode: SelectionMode) -> Selection {
        Selection {
            anchor: start,
            head: end,
            mode,
        }
    }

    pub fn mode(&self) -> SelectionMode {
        self.mode
    }

    pub fn anchor(&self) -> Point {
        self.anchor
    }

    /// Moves the loose end. The anchor stays put, so dragging backwards past
    /// the start selects in the other direction.
    pub fn extend(&mut self, head: Point) {
        self.head = head;
    }

    /// Start and end in reading order, whichever way round they were made.
    pub fn range(&self) -> (Point, Point) {
        if self.anchor <= self.head {
            (self.anchor, self.head)
        } else {
            (self.head, self.anchor)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.anchor == self.head
    }

    /// Whether a cell falls inside the selection. The end column is included,
    /// so clicking one cell selects that cell.
    pub fn contains(&self, row: u32, col: u16) -> bool {
        let (start, end) = self.range();
        if row < start.row || row > end.row {
            return false;
        }
        let after_start = row > start.row || col >= start.col;
        let before_end = row < end.row || col <= end.col;
        after_start && before_end
    }
}
