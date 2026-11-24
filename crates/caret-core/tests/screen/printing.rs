use crate::common::write;
use caret_core::{Color, Flags, Screen};

#[test]
fn starts_with_a_full_viewport_and_no_history() {
    let s = Screen::new(10, 4, 100);
    assert_eq!((s.cols(), s.rows()), (10, 4));
    assert_eq!(s.history_len(), 0);
    assert_eq!(s.scroll_offset(), 0);
}

#[test]
fn clamps_zero_dimensions() {
    let s = Screen::new(0, 0, 0);
    assert_eq!((s.cols(), s.rows()), (1, 1));
}

#[test]
fn printing_advances_the_cursor() {
    let mut s = Screen::new(10, 2, 0);
    write(&mut s, "hi");
    assert_eq!(s.cursor().x, 2);
    assert_eq!(s.row_text(0), "hi");
}

// The deferred wrap: filling the last column must not scroll until another
// character actually arrives.
#[test]
fn wrap_is_deferred_until_the_next_character() {
    let mut s = Screen::new(3, 2, 10);
    write(&mut s, "abc");
    assert_eq!(s.cursor().x, 2, "parks on the last column");
    assert!(s.cursor().pending_wrap);
    assert_eq!(s.cursor().y, 0, "has not moved down yet");

    write(&mut s, "d");
    assert_eq!((s.cursor().x, s.cursor().y), (1, 1));
    assert_eq!(s.row_text(0), "abc");
    assert_eq!(s.row_text(1), "d");
}

#[test]
fn wrapped_rows_are_marked() {
    let mut s = Screen::new(3, 2, 10);
    write(&mut s, "abcd");
    assert!(s.is_wrapped(0), "row 0 continues onto row 1");
    assert!(!s.is_wrapped(1));
}

#[test]
fn no_wrap_mode_overwrites_the_last_column() {
    let mut s = Screen::new(3, 2, 0);
    for ch in "abcd".chars() {
        s.print(ch, 1, false, false);
    }
    assert_eq!(s.cursor().y, 0);
    assert_eq!(s.row_text(0), "abd");
}

#[test]
fn printed_cells_carry_the_pen_but_not_wide_markers() {
    let mut s = Screen::new(4, 1, 0);
    s.set_pen_fg(Color::indexed(5));
    s.pen_flags_mut().insert(Flags::BOLD | Flags::WIDE);
    s.print('x', 1, true, false);

    let c = s.line(0)[0];
    assert_eq!(c.fg, Color::indexed(5));
    assert!(c.flags.contains(Flags::BOLD));
    assert!(
        !c.flags.contains(Flags::WIDE),
        "WIDE is positional, not SGR state"
    );
}

#[test]
fn wide_character_occupies_two_cells() {
    let mut s = Screen::new(6, 1, 0);
    s.print('漢', 2, true, false);
    let row = s.line(0);
    assert_eq!(row[0].ch, '漢');
    assert!(row[0].flags.contains(Flags::WIDE));
    assert!(row[1].flags.contains(Flags::WIDE_TAIL));
    assert_eq!(s.cursor().x, 2);
}

#[test]
fn wide_character_wraps_rather_than_straddling_the_edge() {
    let mut s = Screen::new(3, 2, 10);
    write(&mut s, "ab");
    s.print('漢', 2, true, false);
    assert_eq!(s.cursor().y, 1);
    assert_eq!(s.line(1)[0].ch, '漢');
}

#[test]
fn zero_width_characters_are_dropped() {
    let mut s = Screen::new(4, 1, 0);
    write(&mut s, "a");
    s.print('\u{0301}', 0, true, false);
    assert_eq!(s.cursor().x, 1);
    assert_eq!(s.row_text(0), "a");
}
