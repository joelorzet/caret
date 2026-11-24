#[allow(unused_imports)]
use crate::prelude::*;

/// An open search prompt.
pub struct SearchState {
    /// The pane being searched; matches are meaningless against another.
    pub(crate) pane: PaneId,
    pub(crate) query: String,
    /// Every match, oldest first, as inclusive start and end points.
    pub(crate) matches: Vec<(Point, Point)>,
    pub(crate) current: usize,
}
