mod common;

use caret_layout::{Axis, Direction, PaneId};
use common::{tab, AREA};

#[test]
fn moves_across_a_vertical_divider() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), false);

    assert_eq!(t.neighbour(AREA, Direction::Left), Some(PaneId(0)));
    assert_eq!(t.neighbour(AREA, Direction::Right), None);

    assert!(t.focus_dir(AREA, Direction::Left));
    assert_eq!(t.focus(), PaneId(0));
    assert_eq!(t.neighbour(AREA, Direction::Right), Some(PaneId(1)));
}

#[test]
fn moves_across_a_horizontal_divider() {
    let mut t = tab();
    t.split(Axis::Vertical, PaneId(1), false);

    assert_eq!(t.neighbour(AREA, Direction::Up), Some(PaneId(0)));
    assert_eq!(t.neighbour(AREA, Direction::Down), None);
}

#[test]
fn refuses_to_move_past_the_edge() {
    let mut t = tab();
    assert!(!t.focus_dir(AREA, Direction::Left));
    assert_eq!(t.focus(), PaneId(0));
}

// Layout:  P0 | P1
//             -----
//          P0 | P2
#[test]
fn picks_the_pane_whose_edge_faces_the_current_one() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), false);
    t.split(Axis::Vertical, PaneId(2), false);
    assert_eq!(t.focus(), PaneId(2));

    assert_eq!(t.neighbour(AREA, Direction::Up), Some(PaneId(1)));
    assert_eq!(t.neighbour(AREA, Direction::Left), Some(PaneId(0)));
    assert_eq!(t.neighbour(AREA, Direction::Down), None);
}

// From the tall left pane, moving right must pick the pane that shares the
// most border rather than whichever happens to be first in the tree.
#[test]
fn prefers_the_neighbour_sharing_the_most_border() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), false);
    t.split(Axis::Vertical, PaneId(2), false);

    t.set_focus(PaneId(0));
    let picked = t.neighbour(AREA, Direction::Right).unwrap();
    assert_eq!(picked, PaneId(1), "P1 is 12 rows tall, P2 is 11");
}

#[test]
fn navigation_is_stable_across_three_columns() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), false);
    t.split(Axis::Horizontal, PaneId(2), false);

    t.set_focus(PaneId(1));
    assert_eq!(t.neighbour(AREA, Direction::Left), Some(PaneId(0)));
    assert_eq!(t.neighbour(AREA, Direction::Right), Some(PaneId(2)));
}
