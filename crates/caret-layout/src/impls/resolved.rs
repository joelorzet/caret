#[allow(unused_imports)]
use crate::prelude::*;

/// How far either side of a border counts as grabbing it.
///
/// A border is one cell wide, which is a small target for a pointer, so the
/// cells next to it are treated as part of it.
const GRAB: u16 = 1;

impl Resolved {
    /// The border under a cell, with the pane whose edge it is.
    ///
    /// Resizing moves the focused pane's border, so the caller needs to know
    /// which pane to focus: the one on the near side, left of a vertical
    /// border, above a horizontal one.
    pub fn divider_at(&self, col: u16, row: u16) -> Option<(Divider, PaneId)> {
        let divider = *self.dividers.iter().find(|divider| match divider.axis {
            Axis::Horizontal => {
                near(col, divider.rect.x) && spans(row, divider.rect.y, divider.rect.bottom())
            }
            Axis::Vertical => {
                near(row, divider.rect.y) && spans(col, divider.rect.x, divider.rect.right())
            }
        })?;

        let pane = self.panes.iter().find(|(_, rect)| match divider.axis {
            Axis::Horizontal => {
                rect.right() == divider.rect.x
                    && overlaps(rect.y, rect.bottom(), divider.rect.y, divider.rect.bottom())
            }
            Axis::Vertical => {
                rect.bottom() == divider.rect.y
                    && overlaps(rect.x, rect.right(), divider.rect.x, divider.rect.right())
            }
        })?;

        Some((divider, pane.0))
    }
}

fn near(position: u16, edge: u16) -> bool {
    position + GRAB >= edge && position <= edge + GRAB
}

fn spans(position: u16, start: u16, end: u16) -> bool {
    position >= start && position < end
}

fn overlaps(a_start: u16, a_end: u16, b_start: u16, b_end: u16) -> bool {
    a_start < b_end && b_start < a_end
}
