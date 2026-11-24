mod common;

use caret_layout::{Axis, Direction, PaneId};
use common::{rect_of, tab, AREA};

fn widths(t: &caret_layout::Tab) -> Vec<u16> {
    let r = t.resolve(AREA);
    let mut v: Vec<_> = r.panes.iter().map(|(id, rect)| (id.0, rect.w)).collect();
    v.sort();
    v.into_iter().map(|(_, w)| w).collect()
}

// The rightmost pane has no right border, so resizing it left moves its left
// border left and the pane grows. This is tmux's behaviour.
#[test]
fn resizing_the_rightmost_pane_left_grows_it() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), false);
    assert_eq!(widths(&t), vec![40, 39]);

    assert!(t.resize(AREA, Direction::Left, 5));
    let after = widths(&t);
    assert_eq!(after[0] + after[1], 79, "dividers still accounted for");
    assert!(after[1] > 39, "focused pane grew: {after:?}");
    assert!(
        (after[1] as i32 - 44).abs() <= 1,
        "grew by about 5: {after:?}"
    );
}

#[test]
fn resizing_a_pane_right_shrinks_its_right_neighbour() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), false);
    t.set_focus(PaneId(0));

    assert!(t.resize(AREA, Direction::Right, 6));
    let after = widths(&t);
    assert!(after[0] > 40, "focused pane grew: {after:?}");
    assert_eq!(after[0] + after[1], 79);
}

#[test]
fn resizing_a_pane_left_shrinks_it_when_it_has_a_right_neighbour() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), false);
    t.set_focus(PaneId(0));

    assert!(t.resize(AREA, Direction::Left, 6));
    let after = widths(&t);
    assert!(after[0] < 40, "focused pane shrank: {after:?}");
}

#[test]
fn resizing_along_the_wrong_axis_does_nothing() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), false);
    let before = widths(&t);

    assert!(!t.resize(AREA, Direction::Up, 5));
    assert_eq!(widths(&t), before);
}

#[test]
fn a_single_pane_cannot_be_resized() {
    let mut t = tab();
    assert!(!t.resize(AREA, Direction::Right, 5));
}

#[test]
fn resizing_by_zero_is_a_no_op() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), false);
    assert!(!t.resize(AREA, Direction::Left, 0));
}

#[test]
fn a_zoomed_pane_cannot_be_resized() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), false);
    t.toggle_zoom();
    assert!(!t.resize(AREA, Direction::Left, 5));
}

// Repeated resizing must stop at one cell rather than collapsing a pane or
// letting a weight go negative.
#[test]
fn resizing_stops_before_a_pane_disappears() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), false);

    for _ in 0..200 {
        t.resize(AREA, Direction::Left, 5);
    }
    let after = widths(&t);
    assert!(
        after.iter().all(|&w| w >= 1),
        "no pane collapsed: {after:?}"
    );
    assert_eq!(after[0] + after[1], 79);
}

// With nested splits, the border that moves is the inner one next to the pane,
// not the outer split.
#[test]
fn resizing_uses_the_innermost_matching_split() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), false);
    t.split(Axis::Vertical, PaneId(2), false);

    let left_before = rect_of(&t.resolve(AREA), 0);
    assert!(t.resize(AREA, Direction::Up, 3));

    assert_eq!(
        rect_of(&t.resolve(AREA), 0),
        left_before,
        "the outer horizontal split is untouched"
    );
    assert!(rect_of(&t.resolve(AREA), 2).h > 11);
}
