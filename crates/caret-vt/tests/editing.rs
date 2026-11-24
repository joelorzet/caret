mod common;

use caret_core::Color;
use common::Vt;

#[test]
fn erase_in_display_modes() {
    let mut vt = Vt::new(4, 3);
    vt.feed("abcd\r\nefgh\r\nijkl");

    vt.feed("\x1b[2;3H\x1b[0J");
    assert_eq!(vt.rows(), vec!["abcd", "ef", ""]);

    vt.feed("\x1b[2J");
    assert_eq!(vt.rows(), vec!["", "", ""]);
}

#[test]
fn erase_in_display_to_start() {
    let mut vt = Vt::new(4, 2);
    vt.feed("abcd\r\nefgh");
    vt.feed("\x1b[2;2H\x1b[1J");
    assert_eq!(vt.rows(), vec!["", "  gh"]);
}

#[test]
fn erase_in_line_modes() {
    let mut vt = Vt::new(5, 1);
    vt.feed("abcde\x1b[3G\x1b[0K");
    assert_eq!(vt.row(0), "ab");

    vt.feed("\x1b[1;1Habcde\x1b[3G\x1b[1K");
    assert_eq!(vt.row(0), "   de");

    vt.feed("\x1b[2K");
    assert_eq!(vt.row(0), "");
}

#[test]
fn insert_and_delete_characters() {
    let mut vt = Vt::new(5, 1);
    vt.feed("abcde\x1b[2G\x1b[2@");
    assert_eq!(vt.row(0), "a  bc");

    vt.feed("\x1b[1;1H\x1b[2K").feed("abcde\x1b[2G\x1b[2P");
    assert_eq!(vt.row(0), "ade");
}

#[test]
fn erase_characters_without_shifting() {
    let mut vt = Vt::new(5, 1);
    vt.feed("abcde\x1b[2G\x1b[2X");
    assert_eq!(vt.row(0), "a  de");
}

#[test]
fn insert_and_delete_lines() {
    let mut vt = Vt::new(4, 4);
    vt.feed("aa\r\nbb\r\ncc\r\ndd");

    vt.feed("\x1b[2;1H\x1b[1L");
    assert_eq!(vt.rows(), vec!["aa", "", "bb", "cc"]);

    vt.feed("\x1b[2;1H\x1b[1M");
    assert_eq!(vt.rows(), vec!["aa", "bb", "cc", ""]);
}

#[test]
fn insert_mode_shifts_the_rest_of_the_line() {
    let mut vt = Vt::new(6, 1);
    vt.feed("abcd\x1b[2G\x1b[4hXY");
    assert_eq!(
        vt.row(0),
        "aXYbcd",
        "d is pushed right, and six columns fit it"
    );
}

// Erasing with a background colour set has to paint that colour.
#[test]
fn erase_uses_the_current_background() {
    let mut vt = Vt::new(4, 1);
    vt.feed("\x1b[41m\x1b[2K");
    for x in 0..4 {
        assert_eq!(vt.cell(x, 0).bg, Color::indexed(1), "column {x}");
    }
}

#[test]
fn full_reset_clears_everything() {
    let mut vt = Vt::new(5, 2);
    vt.feed("\x1b[31mabc\x1b[?7l\x1bc");
    assert_eq!(vt.rows(), vec!["", ""]);
    assert_eq!(vt.cursor(), (0, 0));
    assert!(vt.term.wrap_enabled());
    assert_eq!(vt.term.screen().pen().fg, Color::DEFAULT);
}

#[test]
fn soft_reset_keeps_the_screen() {
    let mut vt = Vt::new(5, 2);
    vt.feed("ab\x1b[?7l\x1b[!p");
    assert_eq!(vt.row(0), "ab", "contents survive");
    assert!(vt.term.wrap_enabled(), "modes are reset");
}

#[test]
fn the_alignment_test_fills_the_screen() {
    let mut vt = Vt::new(3, 2);
    vt.feed("\x1b#8");
    assert_eq!(vt.rows(), vec!["EEE", "EEE"]);
    assert_eq!(vt.cursor(), (0, 0));
}
