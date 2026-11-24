mod common;

use common::Vt;

#[test]
fn absolute_positioning_is_one_based() {
    let mut vt = Vt::new(10, 5);
    vt.feed("\x1b[3;4H");
    assert_eq!(vt.cursor(), (3, 2));

    // A missing parameter means 1.
    vt.feed("\x1b[H");
    assert_eq!(vt.cursor(), (0, 0));
}

#[test]
fn relative_motion_in_every_direction() {
    let mut vt = Vt::new(20, 10);
    vt.feed("\x1b[5;5H");
    vt.feed("\x1b[2A");
    assert_eq!(vt.cursor(), (4, 2));
    vt.feed("\x1b[3B");
    assert_eq!(vt.cursor(), (4, 5));
    vt.feed("\x1b[4C");
    assert_eq!(vt.cursor(), (8, 5));
    vt.feed("\x1b[2D");
    assert_eq!(vt.cursor(), (6, 5));
}

#[test]
fn motion_defaults_to_one_and_clamps_at_the_edges() {
    let mut vt = Vt::new(10, 4);
    vt.feed("\x1b[1;1H\x1b[A\x1b[D");
    assert_eq!(vt.cursor(), (0, 0));
    vt.feed("\x1b[100B\x1b[100C");
    assert_eq!(vt.cursor(), (9, 3));
}

#[test]
fn column_and_row_addressing() {
    let mut vt = Vt::new(10, 4);
    vt.feed("\x1b[5G");
    assert_eq!(vt.cursor().0, 4);
    vt.feed("\x1b[3d");
    assert_eq!(vt.cursor().1, 2);
}

#[test]
fn next_and_previous_line_reset_the_column() {
    let mut vt = Vt::new(10, 5);
    vt.feed("\x1b[1;5H\x1b[2E");
    assert_eq!(vt.cursor(), (0, 2));
    vt.feed("\x1b[1;5H\x1b[1F");
    assert_eq!(vt.cursor(), (0, 0));
}

#[test]
fn save_and_restore_the_cursor() {
    let mut vt = Vt::new(10, 4);
    vt.feed("\x1b[2;3H\x1b7\x1b[1;1H\x1b8");
    assert_eq!(vt.cursor(), (2, 1));

    // The CSI form behaves the same way.
    vt.feed("\x1b[4;5H\x1b[s\x1b[1;1H\x1b[u");
    assert_eq!(vt.cursor(), (4, 3));
}

#[test]
fn index_and_reverse_index() {
    let mut vt = Vt::new(10, 3);
    vt.feed("a\x1bDb");
    assert_eq!(vt.row(0), "a");
    assert_eq!(vt.row(1), " b");

    vt.feed("\x1b[1;1H\x1bM");
    assert_eq!(vt.cursor(), (0, 0));
}

#[test]
fn next_line_also_returns_the_carriage() {
    let mut vt = Vt::new(10, 3);
    vt.feed("abc\x1bEx");
    assert_eq!(vt.row(1), "x");
}

#[test]
fn the_cursor_can_be_hidden_and_shown() {
    let mut vt = Vt::new(5, 1);
    assert!(vt.term.cursor_visible());
    vt.feed("\x1b[?25l");
    assert!(!vt.term.cursor_visible());
    vt.feed("\x1b[?25h");
    assert!(vt.term.cursor_visible());
}

#[test]
fn cursor_style_follows_decscusr() {
    use caret_core::CursorShape;
    let mut vt = Vt::new(5, 1);
    vt.feed("\x1b[4 q");
    assert_eq!(vt.term.cursor_style().shape, CursorShape::Underline);
    assert!(!vt.term.cursor_style().blink);

    vt.feed("\x1b[5 q");
    assert_eq!(vt.term.cursor_style().shape, CursorShape::Bar);
    assert!(vt.term.cursor_style().blink);
}
