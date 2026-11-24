use crate::common::{viewport, write};
use caret_core::{Color, EraseMode, Screen};

#[test]
fn insert_cells_shifts_right_and_drops_the_overflow() {
    let mut s = Screen::new(5, 1, 0);
    write(&mut s, "abcde");
    s.move_to(1, 0);
    s.insert_cells(2);
    assert_eq!(s.row_text(0), "a  bc");
}

#[test]
fn delete_cells_shifts_left_and_blanks_the_tail() {
    let mut s = Screen::new(5, 1, 0);
    write(&mut s, "abcde");
    s.move_to(1, 0);
    s.delete_cells(2);
    assert_eq!(s.row_text(0), "ade");
}

#[test]
fn erase_cells_does_not_shift() {
    let mut s = Screen::new(5, 1, 0);
    write(&mut s, "abcde");
    s.move_to(1, 0);
    s.erase_cells(2);
    assert_eq!(s.row_text(0), "a  de");
}

#[test]
fn erase_line_honours_each_mode() {
    let mut s = Screen::new(5, 1, 0);

    write(&mut s, "abcde");
    s.move_to(2, 0);
    s.erase_line(EraseMode::ToEnd);
    assert_eq!(s.row_text(0), "ab");

    s.move_to(0, 0);
    write(&mut s, "abcde");
    s.move_to(2, 0);
    s.erase_line(EraseMode::ToStart);
    assert_eq!(s.row_text(0), "   de");

    s.erase_line(EraseMode::All);
    assert_eq!(s.row_text(0), "");
}

#[test]
fn erase_display_to_end_clears_the_rest_of_the_screen() {
    let mut s = Screen::new(4, 3, 0);
    write(&mut s, "ab\r\ncd\r\nef");
    s.move_to(1, 1);
    s.erase_display(EraseMode::ToEnd);
    assert_eq!(viewport(&s), vec!["ab", "c", ""]);
}

// Erasing with a background set must paint that background, not just blank out
// the characters.
#[test]
fn erase_paints_the_pen_background() {
    let mut s = Screen::new(4, 1, 0);
    let bg = Color::rgb(1, 2, 3);
    s.set_pen_bg(bg);
    s.erase_line(EraseMode::All);
    assert!(s.line(0).iter().all(|c| c.bg == bg));
}
