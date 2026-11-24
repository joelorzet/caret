// Each integration test binary compiles this separately, so some helpers are
// unused in some of them.
#![allow(dead_code)]

use caret_layout::{PaneId, Rect, Resolved, Tab, TabId};

pub const AREA: Rect = Rect::new(0, 0, 80, 24);

pub fn tab() -> Tab {
    Tab::new(TabId(0), PaneId(0))
}

pub fn rect_of(r: &Resolved, pane: u32) -> Rect {
    r.panes
        .iter()
        .find(|(id, _)| *id == PaneId(pane))
        .unwrap_or_else(|| panic!("pane {pane} not in layout"))
        .1
}

pub fn ids(r: &Resolved) -> Vec<u32> {
    r.panes.iter().map(|(id, _)| id.0).collect()
}
