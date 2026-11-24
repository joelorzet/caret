use caret_layout::{LayoutNode, LayoutPreset, LayoutShape, LayoutTemplate, PaneId, Workspace};

use crate::support::template;

// Rearranging must not restart anyone's shell.
#[test]
fn applying_a_layout_reuses_the_panes_already_open() {
    let mut ws = Workspace::new();
    ws.apply_layout(&template(LayoutPreset::MainVertical, Some(3), 50.0));
    let before = ws.active_tab().panes();

    let change = ws.apply_layout(&template(LayoutPreset::Tiled, None, 50.0));
    assert!(change.created.is_empty(), "no new panes: {change:?}");
    assert!(change.removed.is_empty(), "none closed: {change:?}");
    assert_eq!(ws.active_tab().panes(), before);
}

#[test]
fn a_smaller_layout_reports_the_panes_to_close() {
    let mut ws = Workspace::new();
    ws.apply_layout(&template(LayoutPreset::Tiled, Some(4), 50.0));

    let change = ws.apply_layout(&template(LayoutPreset::EvenHorizontal, Some(2), 50.0));
    assert_eq!(change.removed.len(), 2);
    assert!(change.created.is_empty());
    assert_eq!(ws.active_tab().pane_count(), 2);
}

#[test]
fn focus_survives_a_layout_change() {
    let mut ws = Workspace::new();
    ws.apply_layout(&template(LayoutPreset::Tiled, Some(4), 50.0));
    ws.active_tab_mut().set_focus(PaneId(2));

    ws.apply_layout(&template(LayoutPreset::EvenVertical, None, 50.0));
    assert_eq!(ws.active_pane(), PaneId(2));
}

#[test]
fn focus_moves_to_a_survivor_when_its_pane_is_removed() {
    let mut ws = Workspace::new();
    ws.apply_layout(&template(LayoutPreset::Tiled, Some(4), 50.0));
    ws.active_tab_mut().set_focus(PaneId(3));

    ws.apply_layout(&template(LayoutPreset::EvenHorizontal, Some(2), 50.0));
    assert!(ws.active_tab().panes().contains(&ws.active_pane()));
}

#[test]
fn an_explicit_template_ignores_the_current_pane_count() {
    let shape = LayoutNode::parse("h(*, v(*, *))").unwrap();
    let t = LayoutTemplate::new("dev", "", LayoutShape::Explicit(shape));
    assert_eq!(t.pane_count(1), 3);
    assert_eq!(t.pane_count(9), 3);
}
