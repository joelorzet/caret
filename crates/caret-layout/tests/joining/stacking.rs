use crate::support::{rect_of, row_of_three, AREA};
use caret_layout::{Axis, Direction, PaneId, Tab, TabId};

// The case this exists for: two of a row's panes become a column, while the
// third is left where it was.
#[test]
fn joining_leftwards_stacks_the_pane_under_its_neighbour() {
    let mut tab = row_of_three();
    tab.set_focus(PaneId(3));
    assert!(tab.join_pane(AREA, Direction::Left));

    let second = rect_of(&tab, PaneId(2));
    let third = rect_of(&tab, PaneId(3));
    assert_eq!(second.x, third.x, "same column");
    assert_eq!(second.w, third.w);
    assert!(third.y >= second.bottom(), "the mover goes underneath");

    let first = rect_of(&tab, PaneId(1));
    assert!(first.right() <= second.x, "the third pane stays put");
    assert_eq!(first.h, AREA.h, "and keeps its full height");
}

#[test]
fn joining_rightwards_puts_the_pane_on_top() {
    let mut tab = row_of_three();
    tab.set_focus(PaneId(1));
    assert!(tab.join_pane(AREA, Direction::Right));

    let first = rect_of(&tab, PaneId(1));
    let second = rect_of(&tab, PaneId(2));
    assert_eq!(first.x, second.x);
    assert!(first.bottom() <= second.y, "the mover goes above");
}

// Stacked panes joined the other way come back to side by side, so the
// command undoes itself.
#[test]
fn joining_upwards_puts_a_stacked_pair_side_by_side() {
    let mut tab = Tab::new(TabId(1), PaneId(1));
    tab.split(Axis::Vertical, PaneId(2), false);
    tab.set_focus(PaneId(2));
    assert!(tab.join_pane(AREA, Direction::Up));

    let first = rect_of(&tab, PaneId(1));
    let second = rect_of(&tab, PaneId(2));
    assert_eq!(first.y, second.y);
    assert!(second.x >= first.right(), "the mover goes to the right");
}

#[test]
fn no_pane_is_lost_or_duplicated() {
    let mut tab = row_of_three();
    tab.set_focus(PaneId(3));
    tab.join_pane(AREA, Direction::Left);

    let mut panes = tab.panes();
    panes.sort_by_key(|p| p.0);
    assert_eq!(panes, vec![PaneId(1), PaneId(2), PaneId(3)]);
    assert_eq!(tab.resolve(AREA).panes.len(), 3);
}

#[test]
fn the_moved_pane_keeps_the_focus() {
    let mut tab = row_of_three();
    tab.set_focus(PaneId(3));
    tab.join_pane(AREA, Direction::Left);
    assert_eq!(tab.focus(), PaneId(3));
}

#[test]
fn there_is_nothing_to_join_in_a_single_pane_tab() {
    let mut tab = Tab::new(TabId(1), PaneId(1));
    for dir in [
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ] {
        assert!(!tab.join_pane(AREA, dir));
    }
    assert_eq!(tab.pane_count(), 1);
}

// Along the row's own axis there is nowhere further to go, so the leftmost
// pane pushed left stays put. Across it there is: see the pushing tests.
#[test]
fn joining_past_the_end_of_a_row_does_nothing() {
    let mut tab = row_of_three();
    tab.set_focus(PaneId(1));
    assert!(!tab.join_pane(AREA, Direction::Left));
    assert_eq!(tab.resolve(AREA).panes.len(), 3);
    assert_eq!(rect_of(&tab, PaneId(1)).x, AREA.x);
}

// Zoom hides every other pane, so a layout change underneath it would only
// show up on unzooming.
#[test]
fn joining_leaves_zoom() {
    let mut tab = row_of_three();
    tab.set_focus(PaneId(3));
    tab.toggle_zoom();
    assert!(tab.zoomed().is_some());
    tab.join_pane(AREA, Direction::Left);
    assert!(tab.zoomed().is_none());
}
