#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CloseOutcome {
    /// The pane went away and focus moved to the given sibling.
    Pane(PaneId),
    /// It was the tab's last pane, so the tab closed too.
    Tab(TabId),
    /// It was the last pane of the last tab; the window should exit.
    Empty,
    NotFound,
}
