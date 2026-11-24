mod common;

use caret_core::{Point, Screen, Selection, SelectionMode};
use common::write;

const COLS: u16 = 40;

fn screen(text: &str) -> Screen {
    let mut s = Screen::new(COLS, 4, 10);
    write(&mut s, text);
    s
}

#[test]
fn a_fresh_selection_is_empty() {
    let selection = Selection::new(Point::new(0, 0), SelectionMode::Cell);
    assert!(selection.is_empty());
    assert_eq!(selection.range(), (Point::new(0, 0), Point::new(0, 0)));
}

// Dragging backwards must still read forwards.
#[test]
fn the_range_is_normalised_whichever_way_it_was_dragged() {
    let mut selection = Selection::new(Point::new(2, 5), SelectionMode::Cell);
    selection.extend(Point::new(1, 2));
    assert_eq!(selection.range(), (Point::new(1, 2), Point::new(2, 5)));
    assert_eq!(selection.anchor(), Point::new(2, 5), "the anchor stays put");
}

#[test]
fn contains_covers_both_ends() {
    let mut selection = Selection::new(Point::new(1, 2), SelectionMode::Cell);
    selection.extend(Point::new(2, 4));

    assert!(selection.contains(1, 2), "start is included");
    assert!(selection.contains(2, 4), "end is included");
    assert!(!selection.contains(1, 1));
    assert!(!selection.contains(2, 5));
    assert!(
        selection.contains(1, 9),
        "a middle row is covered to its end"
    );
    assert!(selection.contains(2, 0), "and from its start");
    assert!(!selection.contains(0, 5));
    assert!(!selection.contains(3, 0));
}

#[test]
fn a_single_cell_selects_that_cell() {
    let selection = Selection::new(Point::new(0, 3), SelectionMode::Cell);
    assert!(selection.contains(0, 3));
    assert!(!selection.contains(0, 4));
}

#[test]
fn text_from_one_row() {
    let s = screen("hello world");
    let mut selection = Selection::new(Point::new(0, 0), SelectionMode::Cell);
    selection.extend(Point::new(0, 4));
    assert_eq!(s.selection_text(&selection), "hello");
}

#[test]
fn text_across_rows_gets_newlines() {
    let s = screen("one\r\ntwo\r\nthree");
    let mut selection = Selection::new(Point::new(0, 0), SelectionMode::Cell);
    selection.extend(Point::new(2, 4));
    assert_eq!(s.selection_text(&selection), "one\ntwo\nthree");
}

#[test]
fn trailing_blanks_are_trimmed_from_each_row() {
    let s = screen("ab\r\ncd");
    let mut selection = Selection::new(Point::new(0, 0), SelectionMode::Cell);
    selection.extend(Point::new(1, 9));
    assert_eq!(s.selection_text(&selection), "ab\ncd");
}

// A command line that filled the width is one line, not two.
#[test]
fn a_wrapped_line_is_joined_without_a_newline() {
    let mut s = Screen::new(4, 3, 10);
    write(&mut s, "abcdefg");
    assert!(s.is_wrapped(0));

    let mut selection = Selection::new(Point::new(0, 0), SelectionMode::Cell);
    selection.extend(Point::new(1, 3));
    assert_eq!(s.selection_text(&selection), "abcdefg");
}

#[test]
fn a_blank_row_in_the_middle_keeps_its_newline() {
    let s = screen("a\r\n\r\nb");
    let mut selection = Selection::new(Point::new(0, 0), SelectionMode::Cell);
    selection.extend(Point::new(2, 0));
    assert_eq!(s.selection_text(&selection), "a\n\nb");
}

#[test]
fn selecting_scrollback_reads_the_history() {
    let mut s = Screen::new(8, 2, 10);
    for i in 0..5 {
        write(&mut s, &format!("L{i}\r\n"));
    }
    // Logical row 0 is the oldest retained row.
    let mut selection = Selection::new(Point::new(0, 0), SelectionMode::Cell);
    selection.extend(Point::new(1, 1));
    assert_eq!(s.selection_text(&selection), "L0\nL1");
}

#[test]
fn a_range_past_the_end_is_clamped() {
    let s = screen("hi");
    let mut selection = Selection::new(Point::new(0, 0), SelectionMode::Cell);
    selection.extend(Point::new(999, 99));
    assert!(s.selection_text(&selection).starts_with("hi"));
}

// --- double click ----------------------------------------------------------

const SEPARATORS: &str = " \t\"'`()[]{}<>|;:,";

#[test]
fn a_double_click_selects_the_word_under_it() {
    let s = screen("cargo build --release");
    let (start, end) = s.word_at(Point::new(0, 7), SEPARATORS).unwrap();
    assert_eq!((start.col, end.col), (6, 10));

    let mut selection = Selection::spanning(start, end, SelectionMode::Word);
    assert_eq!(s.selection_text(&selection), "build");
    selection.extend(end);
}

#[test]
fn a_word_at_the_start_or_end_of_a_row_is_bounded() {
    let s = screen("alpha omega");
    let (start, _) = s.word_at(Point::new(0, 0), SEPARATORS).unwrap();
    assert_eq!(start.col, 0);

    let (_, end) = s.word_at(Point::new(0, 10), SEPARATORS).unwrap();
    assert_eq!(end.col, 10);
}

#[test]
fn double_clicking_a_separator_selects_nothing() {
    let s = screen("a b");
    assert!(s.word_at(Point::new(0, 1), SEPARATORS).is_none());
    assert!(
        s.word_at(Point::new(0, 5), SEPARATORS).is_none(),
        "blank cell"
    );
}

#[test]
fn separators_are_configurable() {
    let s = screen("src/main.rs");
    // With `/` a separator, only one component is picked out.
    let (start, end) = s.word_at(Point::new(0, 1), "/").unwrap();
    assert_eq!((start.col, end.col), (0, 2));

    // Without it, the whole path is one word.
    let (start, end) = s.word_at(Point::new(0, 1), " ").unwrap();
    assert_eq!((start.col, end.col), (0, 10));
}

// --- triple click ----------------------------------------------------------

#[test]
fn a_triple_click_selects_the_whole_row() {
    let s = screen("one\r\ntwo");
    let (start, end) = s.line_at(Point::new(1, 2));
    assert_eq!(start, Point::new(1, 0));
    assert_eq!(end.row, 1);
    assert_eq!(end.col, COLS - 1);
}

// A wrapped command line is one logical line, so a triple click takes all of it.
#[test]
fn a_triple_click_follows_wrapping_both_ways() {
    let mut s = Screen::new(4, 4, 10);
    write(&mut s, "abcdefghij");

    let (start, end) = s.line_at(Point::new(1, 1));
    assert_eq!(start.row, 0);
    assert_eq!(end.row, 2);

    let selection = Selection::spanning(start, end, SelectionMode::Line);
    assert_eq!(s.selection_text(&selection), "abcdefghij");
}

#[test]
fn display_row_maps_the_view_onto_logical_rows() {
    let mut s = Screen::new(8, 2, 10);
    for i in 0..5 {
        write(&mut s, &format!("L{i}\r\n"));
    }
    assert_eq!(s.display_row(0), s.history_len());

    s.scroll_display(3);
    assert_eq!(s.display_row(0), s.history_len() - 3, "scrolled back");
}
