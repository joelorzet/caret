use caret_core::Screen;

#[test]
fn cursor_motion_respects_the_region_and_edges() {
    let mut s = Screen::new(10, 10, 0);
    s.set_region(2, 7);

    s.move_to(0, 5);
    s.move_up(10);
    assert_eq!(s.cursor().y, 2, "stops at the region top");

    s.move_down(100);
    assert_eq!(s.cursor().y, 7, "stops at the region bottom");

    s.move_left(100);
    assert_eq!(s.cursor().x, 0);
    s.move_right(100);
    assert_eq!(s.cursor().x, 9);
}

#[test]
fn save_and_restore_cursor() {
    let mut s = Screen::new(10, 10, 0);
    s.move_to(4, 6);
    s.save_cursor();
    s.move_to(0, 0);
    s.restore_cursor();
    assert_eq!((s.cursor().x, s.cursor().y), (4, 6));
}
