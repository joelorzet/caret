use crate::support::{rect_of, row_of_three, row_with_a_stacked_pair, AREA};
use caret_layout::{Direction, PaneId};

// Stacking a pair has to be undoable, or a layout is a one-way door. With
// nothing to the right there is nothing to pair with, so the pane leaves the
// stack and becomes a column of its own.
#[test]
fn pushing_right_off_the_edge_returns_a_stacked_pane_to_the_row() {
    let mut tab = row_with_a_stacked_pair();
    tab.set_focus(PaneId(3));
    assert!(tab.join_pane(AREA, Direction::Right));

    let rects = tab.resolve(AREA).panes;
    let y: Vec<u16> = rects.iter().map(|(_, r)| r.y).collect();
    let h: Vec<u16> = rects.iter().map(|(_, r)| r.h).collect();
    assert!(y.iter().all(|v| *v == y[0]), "all three back in one row");
    assert!(h.iter().all(|v| *v == AREA.h), "each full height");
    assert_eq!(tab.focus(), PaneId(3));
}

#[test]
fn the_pushed_pane_lands_on_the_side_it_was_pushed_toward() {
    let mut tab = row_with_a_stacked_pair();
    tab.set_focus(PaneId(2));
    assert!(tab.join_pane(AREA, Direction::Right));

    let order: Vec<PaneId> = tab.resolve(AREA).panes.iter().map(|(id, _)| *id).collect();
    assert_eq!(order, vec![PaneId(1), PaneId(3), PaneId(2)]);
}

#[test]
fn pushing_a_pane_that_is_already_at_that_edge_does_nothing() {
    let mut tab = row_of_three();
    tab.set_focus(PaneId(1));
    assert!(!tab.join_pane(AREA, Direction::Left));
    tab.set_focus(PaneId(3));
    assert!(!tab.join_pane(AREA, Direction::Right));
    assert_eq!(tab.resolve(AREA).panes.len(), 3);
}

// Pushing up out of a row makes the pane a band across the top, with what is
// left of the row beneath it.
#[test]
fn pushing_up_out_of_a_row_spans_the_width() {
    let mut tab = row_of_three();
    tab.set_focus(PaneId(2));
    assert!(tab.join_pane(AREA, Direction::Up));

    let moved = rect_of(&tab, PaneId(2));
    assert_eq!(moved.x, AREA.x);
    assert_eq!(moved.w, AREA.w);
    assert_eq!(moved.y, AREA.y);
    assert!(rect_of(&tab, PaneId(1)).y >= moved.bottom());
}

#[test]
fn pushing_out_never_loses_a_pane() {
    for dir in [
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ] {
        let mut tab = row_with_a_stacked_pair();
        tab.set_focus(PaneId(2));
        tab.join_pane(AREA, dir);
        let mut panes = tab.panes();
        panes.sort_by_key(|p| p.0);
        assert_eq!(panes, vec![PaneId(1), PaneId(2), PaneId(3)], "{dir:?}");
    }
}
