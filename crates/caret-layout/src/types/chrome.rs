#[allow(unused_imports)]
use crate::prelude::*;

/// Window furniture around the pane tree: the tab bar, the status line and the
/// two docks.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Chrome {
    pub tab_bar: u16,
    /// Columns of breathing room either side of the pane area, so content
    /// does not sit against the window edge.
    pub padding: u16,
    pub status: u16,
    pub left: Dock,
    pub right: Dock,
}
