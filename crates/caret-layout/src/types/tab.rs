#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone)]
pub struct Tab {
    pub(crate) id: TabId,
    pub(crate) title: String,
    pub(crate) root: Node,
    pub(crate) focus: PaneId,
    /// While set, only this pane is drawn, at full size.
    pub(crate) zoomed: Option<PaneId>,
    /// Where focus was before the current pane, for the last-pane toggle.
    pub(crate) last_focus: Option<PaneId>,
}
