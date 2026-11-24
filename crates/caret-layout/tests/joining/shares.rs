use crate::support::{rect_of, row_of_three, row_with_a_stacked_pair, AREA};
use caret_layout::{Direction, PaneId};

// Moving a pane used to hand it half of whatever it landed on, so a few moves
// in a row left slivers beside one pane holding nearly everything.
#[test]
fn a_moved_pane_shares_the_split_it_lands_in() {
    let mut tab = row_of_three();
    tab.set_focus(PaneId(3));
    tab.join_pane(AREA, Direction::Left);

    let second = rect_of(&tab, PaneId(2));
    let third = rect_of(&tab, PaneId(3));
    assert!(
        second.h.abs_diff(third.h) <= 1,
        "stacked pair: {} vs {}",
        second.h,
        third.h
    );
}

#[test]
fn moving_a_pane_about_does_not_shrink_it_each_time() {
    let mut tab = row_of_three();
    tab.set_focus(PaneId(2));
    for dir in [
        Direction::Left,
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Up,
        Direction::Right,
    ] {
        tab.join_pane(AREA, dir);
    }

    let smallest = tab
        .resolve(AREA)
        .panes
        .iter()
        .map(|(_, r)| r.w as u32 * r.h as u32)
        .min()
        .expect("panes");
    let whole = AREA.w as u32 * AREA.h as u32;
    assert!(
        smallest * 8 >= whole,
        "smallest pane is {smallest} of {whole} after six moves"
    );
}

#[test]
fn a_pane_pushed_out_to_the_edge_gets_an_even_share() {
    let mut tab = row_with_a_stacked_pair();
    tab.set_focus(PaneId(3));
    tab.join_pane(AREA, Direction::Right);

    let widths: Vec<u16> = tab.resolve(AREA).panes.iter().map(|(_, r)| r.w).collect();
    let (min, max) = (
        *widths.iter().min().expect("panes"),
        *widths.iter().max().expect("panes"),
    );
    assert!(max - min <= 1, "thirds, not slivers: {widths:?}");
}
