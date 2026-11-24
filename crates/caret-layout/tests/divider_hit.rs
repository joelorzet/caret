use caret_layout::{Axis, PaneId, Rect, Tab, TabId};

const AREA: Rect = Rect::new(0, 0, 80, 24);

fn split_side_by_side() -> Tab {
    let mut tab = Tab::new(TabId(1), PaneId(1));
    tab.split(Axis::Horizontal, PaneId(2), false);
    tab
}

#[test]
fn the_border_between_two_panes_is_grabbable() {
    let resolved = split_side_by_side().resolve(AREA);
    let divider = resolved.dividers[0];

    let hit = resolved.divider_at(divider.rect.x, 10);
    assert!(hit.is_some(), "the border itself");
    assert_eq!(hit.unwrap().0, divider);
}

// One cell is a small target for a pointer.
#[test]
fn the_cells_either_side_of_a_border_count_as_the_border() {
    let resolved = split_side_by_side().resolve(AREA);
    let x = resolved.dividers[0].rect.x;

    assert!(resolved.divider_at(x - 1, 10).is_some(), "just left");
    assert!(resolved.divider_at(x + 1, 10).is_some(), "just right");
    assert!(resolved.divider_at(x + 6, 10).is_none(), "well clear of it");
}

// Resizing moves the focused pane's border, so the pane handed back has to be
// the one on the near side or the drag would move the wrong edge.
#[test]
fn the_pane_on_the_near_side_comes_back_with_it() {
    let tab = split_side_by_side();
    let resolved = tab.resolve(AREA);
    let divider = resolved.dividers[0];

    let (_, pane) = resolved.divider_at(divider.rect.x, 10).expect("a hit");
    let (_, rect) = resolved
        .panes
        .iter()
        .find(|(id, _)| *id == pane)
        .expect("the pane");
    assert_eq!(rect.right(), divider.rect.x, "it is the left pane");
}

#[test]
fn a_stacked_split_is_grabbed_by_its_row() {
    let mut tab = Tab::new(TabId(1), PaneId(1));
    tab.split(Axis::Vertical, PaneId(2), false);
    let resolved = tab.resolve(AREA);
    let divider = resolved.dividers[0];
    assert_eq!(divider.axis, Axis::Vertical);

    assert!(resolved.divider_at(40, divider.rect.y).is_some());
    assert!(resolved.divider_at(40, divider.rect.y + 5).is_none());
}

#[test]
fn a_single_pane_has_nothing_to_grab() {
    let tab = Tab::new(TabId(1), PaneId(1));
    let resolved = tab.resolve(AREA);
    assert!(resolved.dividers.is_empty());
    assert!(resolved.divider_at(40, 12).is_none());
}

// A click in the middle of a pane must not start a resize.
#[test]
fn the_middle_of_a_pane_is_not_a_border() {
    let resolved = split_side_by_side().resolve(AREA);
    assert!(resolved.divider_at(10, 12).is_none());
    assert!(resolved.divider_at(70, 12).is_none());
}
