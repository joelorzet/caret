#[allow(unused_imports)]
use crate::prelude::*;

/// What applying a layout did to the set of panes.
///
/// Existing panes are reused in order, so rearranging never restarts a shell.
/// The caller spawns a child for each `created` id and reaps each `removed`.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct LayoutChange {
    pub created: Vec<PaneId>,
    pub removed: Vec<PaneId>,
}
