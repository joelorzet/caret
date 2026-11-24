use caret_app::{arrange_canvas, arrange_rect};

const WINDOW: (u16, u16) = (160, 48);

#[test]
fn the_panel_is_centred_horizontally() {
    let rect = arrange_rect(WINDOW);
    let left = rect.x;
    let right = WINDOW.0 - rect.right();
    assert!(left.abs_diff(right) <= 1, "{left} vs {right}");
}

#[test]
fn the_panel_never_leaves_the_window() {
    for size in [(20u16, 8u16), (40, 12), (160, 48), (400, 120)] {
        let rect = arrange_rect(size);
        assert!(rect.right() <= size.0, "{size:?} gave {rect:?}");
        assert!(rect.bottom() <= size.1, "{size:?} gave {rect:?}");
    }
}

// The diagram is both drawn into this rectangle and clicked in it, so it has
// to sit inside the panel's border with the title and hint rows clear.
#[test]
fn the_canvas_stays_inside_the_panel() {
    let panel = arrange_rect(WINDOW);
    let canvas = arrange_canvas(panel);
    assert!(canvas.x > panel.x);
    assert!(canvas.y > panel.y);
    assert!(canvas.right() < panel.right());
    assert!(canvas.bottom() < panel.bottom() - 1, "room for the hints");
}

#[test]
fn a_window_too_small_still_gives_a_canvas_that_fits() {
    for size in [(10u16, 4u16), (30, 10), (31, 11)] {
        let panel = arrange_rect(size);
        let canvas = arrange_canvas(panel);
        assert!(canvas.right() <= panel.right().max(panel.x));
        assert!(canvas.bottom() <= panel.bottom().max(panel.y));
    }
}

#[test]
fn the_panel_grows_with_the_window_but_stops() {
    let small = arrange_rect((80, 24));
    let large = arrange_rect((400, 120));
    assert!(large.w > small.w);
    assert!(large.w <= 120, "a diagram wider than this reads as a grid");
    assert!(large.h <= 36);
}
