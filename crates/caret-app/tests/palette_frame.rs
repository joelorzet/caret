use caret_app::palette_rect;

const WINDOW: (u16, u16) = (120, 40);

#[test]
fn the_frame_is_centred_horizontally() {
    let rect = palette_rect(WINDOW, 10);
    let left = rect.x;
    let right = WINDOW.0 - rect.right();
    assert!(left.abs_diff(right) <= 1, "{left} vs {right}");
}

// Sitting a third of the way down reads better than dead centre, and matters
// because a click maps a row back through this same offset.
#[test]
fn the_frame_sits_above_centre() {
    let rect = palette_rect(WINDOW, 10);
    assert!(rect.y > 0);
    assert!(rect.y < (WINDOW.1 - rect.h) / 2 + 1);
}

#[test]
fn the_frame_grows_with_the_list() {
    let short = palette_rect(WINDOW, 3);
    let tall = palette_rect(WINDOW, 20);
    assert_eq!(tall.h - short.h, 17);
}

#[test]
fn the_frame_never_leaves_the_window() {
    for rows in [0, 5, 100, 1000] {
        for size in [(120, 40), (40, 12), (20, 6)] {
            let rect = palette_rect(size, rows);
            assert!(rect.right() <= size.0, "{size:?} rows {rows}: too wide");
            assert!(rect.bottom() <= size.1, "{size:?} rows {rows}: too tall");
        }
    }
}

#[test]
fn a_narrow_window_still_gets_a_usable_frame() {
    let rect = palette_rect((30, 10), 4);
    assert!(rect.w >= 20, "got {}", rect.w);
}
