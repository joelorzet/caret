use crate::support::{row_of_three, AREA};
use caret_layout::{Direction, PaneId};

// Escape out of a rearrangement has to put the tab back exactly, not
// approximately: the weights are what a hand-resized layout is made of.
#[test]
fn a_layout_can_be_taken_and_put_back_unchanged() {
    let mut tab = row_of_three();
    tab.set_focus(PaneId(2));
    tab.resize(AREA, Direction::Right, 6);
    let before = tab.layout();
    let focus = tab.focus();
    let rects: Vec<_> = tab.resolve(AREA).panes;

    tab.set_focus(PaneId(3));
    tab.join_pane(AREA, Direction::Left);
    tab.rotate_panes(true);
    assert_ne!(tab.resolve(AREA).panes, rects);

    tab.set_layout(before, focus);
    assert_eq!(tab.resolve(AREA).panes, rects);
    assert_eq!(tab.focus(), focus);
}

// A pane can go away while the layout is held, and putting the old shape back
// must not resurrect it as a focus that no longer exists.
#[test]
fn restoring_falls_back_to_a_pane_that_is_still_there() {
    let mut tab = row_of_three();
    let before = tab.layout();
    tab.close(PaneId(2));

    tab.set_layout(before, PaneId(2));
    assert!(tab.panes().contains(&tab.focus()));
}

#[test]
fn restoring_a_layout_clears_zoom() {
    let mut tab = row_of_three();
    let before = tab.layout();
    tab.toggle_zoom();
    tab.set_layout(before, PaneId(1));
    assert!(tab.zoomed().is_none());
}
