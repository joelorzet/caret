use crate::common::{ids, rect_of, AREA};
use caret_layout::{LayoutPreset, LayoutTemplate, Rect, Workspace};

use crate::support::template;

// The layout asked for: one pane on the left half, two stacked on the right.
#[test]
fn main_vertical_puts_one_pane_left_and_the_rest_stacked_right() {
    let mut ws = Workspace::new();
    let change = ws.apply_layout(&template(LayoutPreset::MainVertical, Some(3), 50.0));

    assert_eq!(change.created.len(), 2, "started with one pane");
    assert!(change.removed.is_empty());

    let r = ws.active_tab().resolve(AREA);
    assert_eq!(ids(&r).len(), 3);
    assert_eq!(rect_of(&r, 0), Rect::new(0, 0, 40, 24));
    assert_eq!(rect_of(&r, 1), Rect::new(41, 0, 39, 12));
    assert_eq!(rect_of(&r, 2), Rect::new(41, 13, 39, 11));
}

#[test]
fn main_horizontal_is_the_same_shape_rotated() {
    let mut ws = Workspace::new();
    ws.apply_layout(&template(LayoutPreset::MainHorizontal, Some(3), 50.0));

    let r = ws.active_tab().resolve(AREA);
    assert_eq!(rect_of(&r, 0), Rect::new(0, 0, 80, 12));
    assert_eq!(rect_of(&r, 1), Rect::new(0, 13, 40, 11));
    assert_eq!(rect_of(&r, 2), Rect::new(41, 13, 39, 11));
}

#[test]
fn the_main_pane_share_is_honoured() {
    let mut ws = Workspace::new();
    ws.apply_layout(&template(LayoutPreset::MainVertical, Some(2), 70.0));
    let r = ws.active_tab().resolve(AREA);
    assert_eq!(rect_of(&r, 0).w, 55, "70% of 79 usable cells");
}

#[test]
fn even_presets_divide_the_window() {
    let mut ws = Workspace::new();
    ws.apply_layout(&template(LayoutPreset::EvenHorizontal, Some(4), 50.0));
    let r = ws.active_tab().resolve(AREA);
    let widths: Vec<u16> = (0..4).map(|i| rect_of(&r, i).w).collect();
    assert_eq!(widths, vec![20, 19, 19, 19]);
    assert_eq!(widths.iter().sum::<u16>() + 3, AREA.w);
}

#[test]
fn tiled_makes_a_grid() {
    let mut ws = Workspace::new();
    ws.apply_layout(&template(LayoutPreset::Tiled, Some(4), 50.0));
    let r = ws.active_tab().resolve(AREA);
    assert_eq!(r.panes.len(), 4);
    // Two rows of two: each pane is about a quarter of the window.
    for (_, rect) in &r.panes {
        assert!(rect.w < AREA.w, "not full width: {rect:?}");
        assert!(rect.h < AREA.h, "not full height: {rect:?}");
    }
}

#[test]
fn a_single_pane_layout_fills_the_window() {
    let mut ws = Workspace::new();
    ws.apply_layout(&template(LayoutPreset::Tiled, Some(1), 50.0));
    assert_eq!(ws.active_tab().resolve(AREA).panes[0].1, AREA);
}

#[test]
fn presets_exist_without_any_configuration() {
    let presets = LayoutTemplate::presets();
    assert_eq!(presets.len(), 5);
    assert!(presets.iter().any(|t| t.name == "main-vertical"));
    for t in &presets {
        assert!(!t.description.is_empty(), "{} has no description", t.name);
    }
}

#[test]
fn preset_names_accept_aliases() {
    assert_eq!(
        LayoutPreset::parse("main-v"),
        Some(LayoutPreset::MainVertical)
    );
    assert_eq!(LayoutPreset::parse("Grid"), Some(LayoutPreset::Tiled));
    assert_eq!(
        LayoutPreset::parse("columns"),
        Some(LayoutPreset::EvenHorizontal)
    );
    assert_eq!(LayoutPreset::parse("nope"), None);
}
