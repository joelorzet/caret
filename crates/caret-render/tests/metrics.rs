use caret_render::{CellMetrics, FontSlant};

#[test]
fn cell_size_comes_from_the_font_and_the_line_height() {
    let m = CellMetrics::new(8.0, 12.0, 4.0, 1.0, 0.0);
    assert_eq!(m.width, 8.0);
    assert_eq!(m.height, 16.0, "ascent plus descent");

    let taller = CellMetrics::new(8.0, 12.0, 4.0, 1.5, 0.0);
    assert_eq!(taller.height, 24.0);
}

#[test]
fn letter_spacing_widens_the_cell() {
    let m = CellMetrics::new(8.0, 12.0, 4.0, 1.0, 2.0);
    assert_eq!(m.width, 10.0);
}

#[test]
fn a_cell_never_collapses_to_nothing() {
    let m = CellMetrics::new(0.0, 0.0, 0.0, 0.0, -100.0);
    assert!(m.width >= 1.0);
    assert!(m.height >= 1.0);
}

#[test]
fn the_grid_size_accounts_for_padding() {
    let m = CellMetrics::new(10.0, 16.0, 4.0, 1.0, 0.0);
    assert_eq!(m.grid_size(1000.0, 1000.0, 0.0), (100, 50));
    assert_eq!(m.grid_size(1000.0, 1000.0, 10.0), (98, 49));
}

#[test]
fn a_tiny_window_still_reports_one_cell() {
    let m = CellMetrics::new(10.0, 16.0, 4.0, 1.0, 0.0);
    assert_eq!(m.grid_size(1.0, 1.0, 0.0), (1, 1));
    assert_eq!(m.grid_size(100.0, 100.0, 500.0), (1, 1));
}

#[test]
fn the_underline_sits_below_the_baseline() {
    let m = CellMetrics::new(8.0, 12.0, 4.0, 1.0, 0.0);
    assert!(m.underline > m.ascent);
    assert!(m.underline < m.height);
}

#[test]
fn slants_are_named_and_classified() {
    assert_eq!(FontSlant::Upright.name(), "upright");
    assert!(!FontSlant::Upright.is_slanted());
    assert!(FontSlant::Italic.is_slanted());
    assert!(FontSlant::Oblique.is_slanted());
}
