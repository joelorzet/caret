mod common;

use caret_layout::{Axis, PaneId, Rect};
use common::{ids, rect_of, tab, AREA};

#[test]
fn horizontal_split_places_panes_side_by_side_with_a_divider() {
    let mut t = tab();
    assert!(t.split(Axis::Horizontal, PaneId(1), false));

    let r = t.resolve(AREA);
    assert_eq!(rect_of(&r, 0), Rect::new(0, 0, 40, 24));
    assert_eq!(rect_of(&r, 1), Rect::new(41, 0, 39, 24));

    assert_eq!(r.dividers.len(), 1);
    assert_eq!(r.dividers[0].rect, Rect::new(40, 0, 1, 24));
    assert_eq!(r.dividers[0].axis, Axis::Horizontal);
}

#[test]
fn vertical_split_stacks_panes() {
    let mut t = tab();
    t.split(Axis::Vertical, PaneId(1), false);

    let r = t.resolve(AREA);
    assert_eq!(rect_of(&r, 0), Rect::new(0, 0, 80, 12));
    assert_eq!(rect_of(&r, 1), Rect::new(0, 13, 80, 11));
    assert_eq!(r.dividers[0].rect, Rect::new(0, 12, 80, 1));
}

#[test]
fn dividers_never_cost_the_window_a_cell() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), false);
    t.split(Axis::Horizontal, PaneId(2), false);

    let r = t.resolve(AREA);
    let widths: u32 = r.panes.iter().map(|(_, p)| p.w as u32).sum();
    assert_eq!(widths + r.dividers.len() as u32, AREA.w as u32);
}

// The new pane takes half of the one it split, as in tmux.
#[test]
fn splitting_twice_halves_the_target_not_the_window() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), false);
    t.split(Axis::Horizontal, PaneId(2), false);

    let r = t.resolve(AREA);
    assert_eq!(ids(&r), vec![0, 1, 2]);
    assert_eq!(rect_of(&r, 0).w, 39);
    assert_eq!(rect_of(&r, 1).w, 20);
    assert_eq!(rect_of(&r, 2).w, 19);
}

// Same-axis splits join the existing split rather than nesting a new one,
// which is what keeps equalize and resize working on the visible row.
#[test]
fn same_axis_splits_stay_siblings() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), false);
    t.split(Axis::Horizontal, PaneId(2), false);

    let caret_layout::Node::Split(sp) = t.root() else {
        panic!("root should be a split");
    };
    assert_eq!(sp.children().len(), 3, "three siblings, not nested pairs");
}

#[test]
fn equalize_gives_every_sibling_the_same_share() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), false);
    t.split(Axis::Horizontal, PaneId(2), false);
    t.equalize();

    let r = t.resolve(AREA);
    assert_eq!(rect_of(&r, 0).w, 26);
    assert_eq!(rect_of(&r, 1).w, 26);
    assert_eq!(rect_of(&r, 2).w, 26);
}

#[test]
fn cross_axis_split_nests() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), false);
    t.split(Axis::Vertical, PaneId(2), false);

    let r = t.resolve(AREA);
    assert_eq!(rect_of(&r, 0), Rect::new(0, 0, 40, 24));
    assert_eq!(rect_of(&r, 1), Rect::new(41, 0, 39, 12));
    assert_eq!(rect_of(&r, 2), Rect::new(41, 13, 39, 11));
}

#[test]
fn split_before_puts_the_new_pane_first() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), true);
    let r = t.resolve(AREA);
    assert_eq!(ids(&r), vec![1, 0]);
}

#[test]
fn splitting_focuses_the_new_pane_and_cancels_zoom() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), false);
    assert_eq!(t.focus(), PaneId(1));
    t.toggle_zoom();
    t.split(Axis::Horizontal, PaneId(2), false);
    assert_eq!(t.zoomed(), None);
}

#[test]
fn closing_a_pane_collapses_the_split() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), false);

    assert_eq!(t.close(PaneId(1)), Some(PaneId(0)));
    assert_eq!(t.pane_count(), 1);
    assert_eq!(t.resolve(AREA).panes[0].1, AREA);
    assert!(t.resolve(AREA).dividers.is_empty());
}

#[test]
fn closing_the_last_pane_reports_the_tab_as_empty() {
    let mut t = tab();
    assert_eq!(t.close(PaneId(0)), None);
}

#[test]
fn closing_an_unknown_pane_is_a_no_op() {
    let mut t = tab();
    assert_eq!(t.close(PaneId(99)), None);
    assert_eq!(t.pane_count(), 1);
}

#[test]
fn closing_a_middle_pane_shares_its_space_proportionally() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), false);
    t.split(Axis::Horizontal, PaneId(2), false);

    t.close(PaneId(1));
    let r = t.resolve(AREA);
    assert_eq!(ids(&r), vec![0, 2]);
    // P0 held twice P2's share before the close, and still does.
    let (a, b) = (rect_of(&r, 0).w, rect_of(&r, 2).w);
    assert_eq!(a + b + 1, AREA.w);
    assert!(a > b * 3 / 2, "expected roughly 2:1, got {a}:{b}");
}

#[test]
fn closing_the_focused_pane_moves_focus_to_a_sibling() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), false);
    t.split(Axis::Horizontal, PaneId(2), false);
    assert_eq!(t.focus(), PaneId(2));

    t.close(PaneId(2));
    assert_eq!(t.focus(), PaneId(1));
}

#[test]
fn zoom_hides_the_other_panes() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), false);

    assert!(t.toggle_zoom());
    let r = t.resolve(AREA);
    assert_eq!(r.panes.len(), 1);
    assert_eq!(r.panes[0], (PaneId(1), AREA));

    assert!(!t.toggle_zoom());
    assert_eq!(t.resolve(AREA).panes.len(), 2);
}

#[test]
fn zoom_is_pointless_with_one_pane() {
    let mut t = tab();
    assert!(!t.toggle_zoom());
    assert_eq!(t.zoomed(), None);
}

#[test]
fn swap_exchanges_two_panes() {
    let mut t = tab();
    t.split(Axis::Horizontal, PaneId(1), false);
    let before = rect_of(&t.resolve(AREA), 0);

    assert!(t.swap(PaneId(0), PaneId(1)));
    assert_eq!(rect_of(&t.resolve(AREA), 1), before);
}
