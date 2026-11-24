#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DragTarget {
    /// The border between two panes. The pane on the near side of it is
    /// focused when the drag starts, which is what makes the resize apply to
    /// the right border.
    Pane(Axis),
    Dock(DockKind),
}
