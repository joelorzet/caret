mod common;

use caret_layout::{Axis, CloseOutcome, PaneId, TabId, Workspace};

#[test]
fn starts_with_one_tab_and_one_pane() {
    let ws = Workspace::new();
    assert_eq!(ws.tab_count(), 1);
    assert_eq!(ws.active_pane(), PaneId(0));
    assert_eq!(ws.panes(), vec![PaneId(0)]);
}

#[test]
fn pane_ids_are_never_reused() {
    let mut ws = Workspace::new();
    let a = ws.split(Axis::Horizontal, false).unwrap();
    ws.close_pane(a);
    let b = ws.split(Axis::Horizontal, false).unwrap();
    assert_ne!(a, b, "a recycled id would attach to the wrong terminal");
}

#[test]
fn new_tab_becomes_active_with_a_fresh_pane() {
    let mut ws = Workspace::new();
    let (tab, pane) = ws.new_tab();
    assert_eq!(tab, TabId(1));
    assert_eq!(pane, PaneId(1));
    assert_eq!(ws.tab_count(), 2);
    assert_eq!(ws.active_index(), 1);
    assert_eq!(ws.active_pane(), PaneId(1));
}

#[test]
fn splitting_affects_only_the_active_tab() {
    let mut ws = Workspace::new();
    ws.new_tab();
    ws.split(Axis::Horizontal, false);

    assert_eq!(ws.tabs()[0].pane_count(), 1);
    assert_eq!(ws.tabs()[1].pane_count(), 2);
}

#[test]
fn closing_the_last_pane_of_a_tab_closes_the_tab() {
    let mut ws = Workspace::new();
    let (tab, pane) = ws.new_tab();
    assert_eq!(ws.close_pane(pane), CloseOutcome::Tab(tab));
    assert_eq!(ws.tab_count(), 1);
    assert_eq!(ws.active_index(), 0);
}

#[test]
fn closing_the_last_pane_of_the_last_tab_empties_the_workspace() {
    let mut ws = Workspace::new();
    assert_eq!(ws.close_pane(PaneId(0)), CloseOutcome::Empty);
    assert_eq!(ws.tab_count(), 0);
}

#[test]
fn closing_a_pane_reports_the_next_focus() {
    let mut ws = Workspace::new();
    let second = ws.split(Axis::Horizontal, false).unwrap();
    assert_eq!(ws.close_pane(second), CloseOutcome::Pane(PaneId(0)));
}

#[test]
fn closing_an_unknown_pane_is_reported() {
    let mut ws = Workspace::new();
    assert_eq!(ws.close_pane(PaneId(42)), CloseOutcome::NotFound);
}

#[test]
fn tab_selection_wraps_in_both_directions() {
    let mut ws = Workspace::new();
    ws.new_tab();
    ws.new_tab();
    assert_eq!(ws.active_index(), 2);

    ws.next_tab();
    assert_eq!(ws.active_index(), 0);
    ws.prev_tab();
    assert_eq!(ws.active_index(), 2);

    assert!(ws.select_tab(1));
    assert_eq!(ws.active_index(), 1);
    assert!(!ws.select_tab(9));
    assert_eq!(ws.active_index(), 1);
}

#[test]
fn tab_of_finds_the_owning_tab() {
    let mut ws = Workspace::new();
    let (tab, pane) = ws.new_tab();
    assert_eq!(ws.tab_of(pane), Some(tab));
    assert_eq!(ws.tab_of(PaneId(0)), Some(TabId(0)));
    assert_eq!(ws.tab_of(PaneId(99)), None);
}
