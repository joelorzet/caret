use caret_app::assign_pane_numbers;
use caret_layout::PaneId;

fn numbers(slots: &[Option<PaneId>]) -> Vec<(u32, usize)> {
    slots
        .iter()
        .enumerate()
        .filter_map(|(slot, held)| held.map(|id| (id.0, slot + 1)))
        .collect()
}

#[test]
fn panes_are_numbered_from_one_in_the_order_they_arrive() {
    let mut slots = Vec::new();
    assign_pane_numbers(&mut slots, &[PaneId(7), PaneId(3), PaneId(9)]);
    assert_eq!(numbers(&slots), [(7, 1), (3, 2), (9, 3)]);
}

// The whole point: a pane moved about in the arrange panel keeps its number,
// so 2 stays 2 wherever it lands.
#[test]
fn rearranging_does_not_renumber_anything() {
    let mut slots = Vec::new();
    let panes = [PaneId(1), PaneId(2), PaneId(3)];
    assign_pane_numbers(&mut slots, &panes);
    let before = numbers(&slots);

    // Same panes, different order, as a move leaves them.
    assign_pane_numbers(&mut slots, &[PaneId(3), PaneId(1), PaneId(2)]);
    assert_eq!(numbers(&slots), before);
}

#[test]
fn reopening_the_panel_shows_the_same_numbers() {
    let mut slots = Vec::new();
    let panes = [PaneId(4), PaneId(5)];
    assign_pane_numbers(&mut slots, &panes);
    let first = numbers(&slots);
    for _ in 0..5 {
        assign_pane_numbers(&mut slots, &panes);
    }
    assert_eq!(numbers(&slots), first);
}

#[test]
fn a_closed_pane_gives_its_number_up_for_the_next_one() {
    let mut slots = Vec::new();
    assign_pane_numbers(&mut slots, &[PaneId(1), PaneId(2), PaneId(3)]);
    assign_pane_numbers(&mut slots, &[PaneId(1), PaneId(3)]);
    assert_eq!(numbers(&slots), [(1, 1), (3, 3)], "3 keeps its number");

    assign_pane_numbers(&mut slots, &[PaneId(1), PaneId(3), PaneId(8)]);
    assert_eq!(numbers(&slots), [(1, 1), (8, 2), (3, 3)], "8 takes the gap");
}

#[test]
fn no_number_is_handed_out_twice() {
    let mut slots = Vec::new();
    for step in 0..20u32 {
        let live: Vec<PaneId> = (0..=step).filter(|n| n % 3 != 0).map(PaneId).collect();
        assign_pane_numbers(&mut slots, &live);
        let held: Vec<u32> = slots.iter().flatten().map(|id| id.0).collect();
        let mut unique = held.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(held.len(), unique.len(), "a pane is in two slots");
        assert_eq!(held.len(), live.len(), "every live pane has a number");
    }
}

// Closing everything must not leave a tail of empty slots that pushes the
// next pane's number up.
#[test]
fn numbers_start_again_once_every_pane_has_gone() {
    let mut slots = Vec::new();
    assign_pane_numbers(&mut slots, &[PaneId(1), PaneId(2), PaneId(3)]);
    assign_pane_numbers(&mut slots, &[]);
    assert!(slots.is_empty());
    assign_pane_numbers(&mut slots, &[PaneId(9)]);
    assert_eq!(numbers(&slots), [(9, 1)]);
}
