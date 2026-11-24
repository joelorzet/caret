mod common;

use caret_core::Modes;
use common::Vt;

#[test]
fn prints_text_and_advances() {
    let mut vt = Vt::new(10, 2);
    vt.feed("hello");
    assert_eq!(vt.row(0), "hello");
    assert_eq!(vt.cursor(), (5, 0));
}

#[test]
fn carriage_return_and_line_feed() {
    let mut vt = Vt::new(10, 3);
    vt.feed("one\r\ntwo");
    assert_eq!(vt.row(0), "one");
    assert_eq!(vt.row(1), "two");
}

#[test]
fn backspace_moves_without_erasing() {
    let mut vt = Vt::new(10, 1);
    vt.feed("abc\x08X");
    assert_eq!(vt.row(0), "abX");
}

#[test]
fn text_wraps_at_the_right_edge() {
    let mut vt = Vt::new(3, 2);
    vt.feed("abcdef");
    assert_eq!(vt.row(0), "abc");
    assert_eq!(vt.row(1), "def");
}

// DECAWM off: the last column is overwritten instead of wrapping.
#[test]
fn wrapping_can_be_turned_off() {
    let mut vt = Vt::new(3, 2);
    vt.feed("\x1b[?7l").feed("abcdef");
    assert_eq!(vt.row(0), "abf");
    assert_eq!(vt.row(1), "");
    assert!(!vt.term.modes().contains(Modes::WRAP));
}

#[test]
fn utf8_split_across_writes_is_reassembled() {
    let mut vt = Vt::new(10, 1);
    let bytes = "é".as_bytes();
    vt.parser.advance(&mut vt.term, &bytes[..1]);
    vt.parser.advance(&mut vt.term, &bytes[1..]);
    assert_eq!(vt.row(0), "é");
}

#[test]
fn an_escape_sequence_split_across_writes_still_works() {
    let mut vt = Vt::new(10, 2);
    vt.parser.advance(&mut vt.term, b"\x1b[2");
    vt.parser.advance(&mut vt.term, b";3Hx");
    assert_eq!(vt.cursor(), (3, 1));
    assert_eq!(vt.row(1), "  x");
}

#[test]
fn wide_characters_take_two_columns() {
    let mut vt = Vt::new(6, 1);
    vt.feed("漢字");
    assert_eq!(vt.cursor(), (4, 0));
    assert_eq!(vt.cell(0, 0).ch, '漢');
    assert_eq!(vt.cell(2, 0).ch, '字');
}

#[test]
fn tabs_advance_to_the_next_stop() {
    let mut vt = Vt::new(20, 1);
    vt.feed("a\tb");
    assert_eq!(vt.cursor(), (9, 0));
    assert_eq!(vt.row(0), "a       b");
}

#[test]
fn tab_stops_can_be_set_and_cleared() {
    let mut vt = Vt::new(20, 1);
    // Clear all stops, set one at column 4, then tab to it.
    vt.feed("\x1b[3g").feed("\x1b[5G\x1bH").feed("\r\t");
    assert_eq!(vt.cursor().0, 4);
}

#[test]
fn bell_is_reported_once() {
    let mut vt = Vt::new(5, 1);
    vt.feed("a\x07b");
    let events: Vec<_> = vt.term.drain_events().collect();
    assert_eq!(events, vec![caret_core::TerminalEvent::Bell]);
    assert_eq!(vt.row(0), "ab");
}
