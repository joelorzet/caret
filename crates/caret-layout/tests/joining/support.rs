use caret_layout::{Axis, Direction, PaneId, Rect, Tab, TabId};

pub const AREA: Rect = Rect::new(0, 0, 90, 30);

/// Three panes in a row, the way `even-horizontal` leaves them.
pub fn row_of_three() -> Tab {
    let mut tab = Tab::new(TabId(1), PaneId(1));
    tab.split(Axis::Horizontal, PaneId(2), false);
    tab.split(Axis::Horizontal, PaneId(3), false);
    tab
}

pub fn rect_of(tab: &Tab, pane: PaneId) -> Rect {
    tab.resolve(AREA)
        .panes
        .into_iter()
        .find(|(id, _)| *id == pane)
        .expect("pane is in the tab")
        .1
}

/// A row whose last two panes have been stacked: `1 | (2 over 3)`.
pub fn row_with_a_stacked_pair() -> Tab {
    let mut tab = row_of_three();
    tab.set_focus(PaneId(3));
    tab.join_pane(AREA, Direction::Left);
    tab
}
