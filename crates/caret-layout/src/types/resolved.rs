#[allow(unused_imports)]
use crate::prelude::*;

/// The geometry a pane tree works out to for a given area.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Resolved {
    pub panes: Vec<(PaneId, Rect)>,
    pub dividers: Vec<Divider>,
}
