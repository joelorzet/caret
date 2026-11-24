use caret_layout::{LayoutPreset, PaneId, Workspace};

use crate::support::template;

#[test]
fn panes_can_be_reached_by_number() {
    let mut ws = Workspace::new();
    ws.apply_layout(&template(LayoutPreset::Tiled, Some(4), 50.0));
    let tab = ws.active_tab_mut();

    assert!(tab.focus_nth(3));
    assert_eq!(tab.focus(), PaneId(2));
    assert_eq!(tab.focus_index(), 3);

    assert!(!tab.focus_nth(0), "numbering starts at 1");
    assert!(!tab.focus_nth(9));
}

#[test]
fn focus_cycles_in_both_directions() {
    let mut ws = Workspace::new();
    ws.apply_layout(&template(LayoutPreset::EvenHorizontal, Some(3), 50.0));
    let tab = ws.active_tab_mut();
    tab.focus_nth(1);

    tab.focus_next();
    assert_eq!(tab.focus(), PaneId(1));
    tab.focus_next();
    assert_eq!(tab.focus(), PaneId(2));
    tab.focus_next();
    assert_eq!(tab.focus(), PaneId(0), "wraps");
    tab.focus_prev();
    assert_eq!(tab.focus(), PaneId(2), "wraps the other way");
}

#[test]
fn a_lone_pane_has_nowhere_to_cycle() {
    let mut ws = Workspace::new();
    let tab = ws.active_tab_mut();
    assert!(!tab.focus_next());
    assert!(!tab.focus_prev());
}

#[test]
fn last_pane_toggles_back() {
    let mut ws = Workspace::new();
    ws.apply_layout(&template(LayoutPreset::EvenHorizontal, Some(3), 50.0));
    let tab = ws.active_tab_mut();

    assert!(!tab.focus_last(), "nothing to go back to yet");
    tab.focus_nth(1);
    tab.focus_nth(3);
    assert!(tab.focus_last());
    assert_eq!(tab.focus(), PaneId(0));
    assert!(tab.focus_last());
    assert_eq!(tab.focus(), PaneId(2), "toggles");
}

#[test]
fn focusing_the_current_pane_does_not_disturb_the_history() {
    let mut ws = Workspace::new();
    ws.apply_layout(&template(LayoutPreset::EvenHorizontal, Some(2), 50.0));
    let tab = ws.active_tab_mut();
    tab.focus_nth(1);
    tab.focus_nth(2);
    tab.focus_nth(2);
    assert_eq!(tab.last_focus(), Some(PaneId(0)));
}
