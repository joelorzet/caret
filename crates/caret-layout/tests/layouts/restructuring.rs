use crate::common::{ids, AREA};
use caret_layout::{LayoutPreset, PaneId, Workspace};

use crate::support::template;

// Three columns, where the last two should become a stacked pair: the
// main-vertical preset is exactly that shape.
#[test]
fn main_vertical_turns_three_columns_into_a_half_and_a_stack() {
    let mut ws = Workspace::new();
    ws.apply_layout(&template(LayoutPreset::EvenHorizontal, Some(3), 50.0));
    assert_eq!(ws.active_tab().root().to_spec(), "h(*, *, *)");

    ws.apply_layout(&template(LayoutPreset::MainVertical, None, 50.0));
    assert_eq!(ws.active_tab().root().to_spec(), "h(*, v(*, *))");
}

#[test]
fn flipping_turns_a_row_into_a_column() {
    let mut t = crate::common::tab();
    t.split(caret_layout::Axis::Horizontal, PaneId(1), false);
    assert_eq!(t.root().to_spec(), "h(*, *)");

    assert!(t.flip_split());
    assert_eq!(t.root().to_spec(), "v(*, *)");

    assert!(t.flip_split(), "and back again");
    assert_eq!(t.root().to_spec(), "h(*, *)");
}

#[test]
fn flipping_affects_only_the_split_the_pane_is_in() {
    let mut t = crate::common::tab();
    t.split(caret_layout::Axis::Horizontal, PaneId(1), false);
    t.split(caret_layout::Axis::Vertical, PaneId(2), false);
    assert_eq!(t.root().to_spec(), "h(*, v(*, *))");

    // Focus is on pane 2, inside the inner vertical split.
    assert!(t.flip_split());
    assert_eq!(
        t.root().to_spec(),
        "h(*, h(*, *))",
        "the outer split is untouched"
    );
}

#[test]
fn a_lone_pane_has_no_split_to_flip() {
    let mut t = crate::common::tab();
    assert!(!t.flip_split());
}

#[test]
fn flipping_keeps_the_shares() {
    let mut t = crate::common::tab();
    t.split(caret_layout::Axis::Horizontal, PaneId(1), false);
    t.resize(AREA, caret_layout::Direction::Left, 10);
    let before = t.root().to_spec();

    t.flip_split();
    assert_eq!(
        t.root().to_spec(),
        before.replacen('h', "v", 1),
        "only the axis changed"
    );
}

#[test]
fn rotating_moves_panes_through_the_arrangement() {
    let mut t = crate::common::tab();
    t.split(caret_layout::Axis::Horizontal, PaneId(1), false);
    t.split(caret_layout::Axis::Horizontal, PaneId(2), false);
    assert_eq!(ids(&t.resolve(AREA)), vec![0, 1, 2]);

    assert!(t.rotate_panes(true));
    assert_eq!(ids(&t.resolve(AREA)), vec![2, 0, 1], "each moves one along");

    assert!(t.rotate_panes(false));
    assert_eq!(ids(&t.resolve(AREA)), vec![0, 1, 2], "and back");
}

#[test]
fn rotating_keeps_the_shape() {
    let mut t = crate::common::tab();
    t.split(caret_layout::Axis::Horizontal, PaneId(1), false);
    t.split(caret_layout::Axis::Vertical, PaneId(2), false);
    let shape = t.root().to_spec();

    t.rotate_panes(true);
    assert_eq!(t.root().to_spec(), shape, "the arrangement did not move");
    assert_eq!(t.pane_count(), 3);
}

#[test]
fn a_lone_pane_has_nothing_to_rotate() {
    let mut t = crate::common::tab();
    assert!(!t.rotate_panes(true));
}

#[test]
fn restructuring_cancels_zoom() {
    let mut t = crate::common::tab();
    t.split(caret_layout::Axis::Horizontal, PaneId(1), false);

    t.toggle_zoom();
    t.flip_split();
    assert_eq!(t.zoomed(), None, "a hidden pane would be confusing");

    t.toggle_zoom();
    t.rotate_panes(true);
    assert_eq!(t.zoomed(), None);
}
