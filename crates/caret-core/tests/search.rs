mod common;

use caret_core::{Point, Screen};
use common::write;

fn screen_with(lines: &[&str]) -> Screen {
    let mut s = Screen::new(40, 3, 50);
    for line in lines {
        write(&mut s, line);
        write(&mut s, "\r\n");
    }
    s
}

#[test]
fn finds_every_occurrence_oldest_first() {
    let s = screen_with(&["alpha beta", "gamma", "beta delta"]);
    let hits = s.search_all("beta", false);
    assert_eq!(hits.len(), 2);
    assert!(hits[0].0.row < hits[1].0.row, "oldest first");
    assert_eq!(hits[0].0.col, 6);
    assert_eq!(hits[1].0.col, 0);
}

#[test]
fn reports_the_span_of_each_match() {
    let s = screen_with(&["xx hello xx"]);
    let (start, end) = s.search_all("hello", false)[0];
    assert_eq!(start.col, 3);
    assert_eq!(end.col, 7, "inclusive end");
    assert_eq!(start.row, end.row);
}

#[test]
fn search_covers_the_scrollback_not_just_the_screen() {
    let mut s = Screen::new(20, 2, 50);
    for i in 0..20 {
        write(&mut s, &format!("row {i}\r\n"));
    }
    assert!(s.history_len() > 0);
    let hits = s.search_all("row 3", false);
    assert_eq!(
        hits.len(),
        1,
        "a row scrolled out of view is still searched"
    );
}

#[test]
fn case_can_be_ignored_or_respected() {
    let s = screen_with(&["Hello HELLO hello"]);
    assert_eq!(s.search_all("hello", false).len(), 3);
    assert_eq!(s.search_all("hello", true).len(), 1);
    assert_eq!(s.search_all("HELLO", true).len(), 1);
}

#[test]
fn overlapping_occurrences_are_all_reported() {
    let s = screen_with(&["aaaa"]);
    assert_eq!(s.search_all("aa", true).len(), 3);
}

#[test]
fn an_empty_or_missing_needle_finds_nothing() {
    let s = screen_with(&["content"]);
    assert!(s.search_all("", false).is_empty());
    assert!(s.search_all("absent", false).is_empty());
}

#[test]
fn a_needle_longer_than_the_row_finds_nothing() {
    let mut s = Screen::new(4, 2, 10);
    write(&mut s, "ab");
    assert!(s.search_all("abcdefghij", false).is_empty());
}

// A match is confined to one row, and a wrapped line is stored as several, so
// an occurrence straddling the wrap is missed. Pinned here so the behaviour is
// a known limitation rather than a surprise.
#[test]
fn a_match_across_a_wrap_is_not_found() {
    let mut s = Screen::new(6, 3, 10);
    write(&mut s, "abcdefghij");
    assert!(s.is_wrapped(0));
    assert!(
        s.search_all("fg", false).is_empty(),
        "f ends row 0, g starts row 1"
    );
    assert_eq!(
        s.search_all("abcdef", false).len(),
        1,
        "within a row it is found"
    );
}

#[test]
fn spaces_in_the_needle_are_matched() {
    let s = screen_with(&["cargo build --release"]);
    let hits = s.search_all("build --release", false);
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].0.col, 6);
}

#[test]
fn wide_characters_do_not_offset_the_columns() {
    let mut s = Screen::new(20, 2, 10);
    s.print('漢', 2, true, false);
    write(&mut s, "needle");
    let hits = s.search_all("needle", false);
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].0.col, 2, "the wide glyph occupies two columns");
}

#[test]
fn scrolling_to_a_match_brings_it_into_view_with_context() {
    let mut s = Screen::new(20, 6, 50);
    for i in 0..30 {
        write(&mut s, &format!("row {i}\r\n"));
    }
    let (start, _) = s.search_all("row 5", false)[0];
    s.scroll_to_row(start.row);

    let base = s.history_len() - s.scroll_offset();
    let visible = base..base + s.rows() as u32;
    assert!(visible.contains(&start.row), "the match is on screen");
    assert!(start.row > base, "with context above it");
}

#[test]
fn scrolling_to_a_visible_row_does_not_move_the_view() {
    let mut s = Screen::new(20, 4, 50);
    write(&mut s, "only");
    s.scroll_to_row(0);
    assert_eq!(s.scroll_offset(), 0);
}

#[test]
fn matches_can_be_turned_into_a_selection() {
    use caret_core::{Selection, SelectionMode};
    let s = screen_with(&["find me here"]);
    let (start, end) = s.search_all("me", false)[0];
    let selection = Selection::spanning(start, end, SelectionMode::Cell);
    assert_eq!(s.selection_text(&selection), "me");
    assert!(selection.contains(start.row, Point::new(0, 5).col));
}
