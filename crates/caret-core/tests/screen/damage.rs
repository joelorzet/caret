use crate::common::{viewport, write};
use caret_core::{Color, Screen};

#[test]
fn damage_is_tracked_per_row_then_cleared() {
    let mut s = Screen::new(8, 3, 0);
    s.clear_damage();
    assert!(!s.is_damaged(0));

    s.move_to(0, 1);
    s.print('x', 1, true, false);
    assert!(s.is_damaged(1));
    assert!(!s.is_damaged(0));

    s.clear_damage();
    assert!(!s.is_damaged(1));

    s.damage_all();
    assert!((0..3).all(|y| s.is_damaged(y)));
}

#[test]
fn reset_clears_grid_history_and_pen() {
    let mut s = Screen::new(8, 2, 10);
    s.set_pen_fg(Color::indexed(3));
    for i in 0..6 {
        write(&mut s, &format!("L{i}\r\n"));
    }
    s.reset();

    assert_eq!(s.history_len(), 0);
    assert_eq!(viewport(&s), vec!["", ""]);
    assert_eq!(s.cursor().x, 0);
    assert_eq!(s.pen().fg, Color::DEFAULT);
}
