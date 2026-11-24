use crate::common::write;
use caret_core::{Flags, Point, Screen, Selection, SelectionMode};

/// Writes `text` and then hangs `marks` on the last character of it, which
/// is what the parser does when a mark arrives with no precomposed form.
fn with_marks(s: &mut Screen, text: &str, marks: &[char]) {
    write(s, text);
    for mark in marks {
        assert!(s.attach(*mark), "the mark found a letter to land on");
    }
}

fn row(s: &Screen) -> u32 {
    s.history_len() + s.cursor().y as u32
}

fn cell_text(s: &Screen, l: u32, col: u16) -> String {
    s.grapheme_at(l, col)
        .map(|g| g.to_string())
        .unwrap_or_default()
}

#[test]
fn a_mark_shares_the_cell_of_the_letter_it_belongs_to() {
    let mut s = Screen::new(20, 5, 100);
    let l = row(&s);
    with_marks(&mut s, "ab", &['\u{0330}']);

    assert_eq!(cell_text(&s, l, 1), "b\u{0330}");
    assert_eq!(s.cursor().x, 2, "the mark took no column of its own");
    assert_eq!(s.row_text(l), "ab\u{0330}");
}

#[test]
fn a_mark_with_nothing_printed_before_it_has_nowhere_to_go() {
    let mut s = Screen::new(20, 5, 100);
    assert!(!s.attach('\u{0301}'));
    assert_eq!(s.cursor().x, 0);
}

#[test]
fn a_cell_that_was_given_a_precomposed_form_is_one_character_again() {
    let mut s = Screen::new(20, 5, 100);
    let l = row(&s);
    with_marks(&mut s, "e", &['\u{0301}']);
    assert!(s.grapheme_at(l, 0).is_some_and(|g| g.is_cluster()));

    assert!(s.recompose('é'));
    assert_eq!(cell_text(&s, l, 0), "é");
    assert!(
        s.grapheme_at(l, 0).is_some_and(|g| !g.is_cluster()),
        "and holds nothing beside the grid any more"
    );
}

#[test]
fn copying_a_cluster_takes_the_marks_with_it() {
    let mut s = Screen::new(20, 5, 100);
    let l = row(&s);
    with_marks(&mut s, "que", &['\u{0330}']);

    let text = s.selection_text(&Selection::spanning(
        Point::new(l, 0),
        Point::new(l, 2),
        SelectionMode::Cell,
    ));
    assert_eq!(text, "que\u{0330}");
}

#[test]
fn a_word_runs_through_a_cluster_rather_than_stopping_at_it() {
    let mut s = Screen::new(20, 5, 100);
    let l = row(&s);
    with_marks(&mut s, "sal", &['\u{0330}']);
    write(&mut s, "ud box");

    let (start, end) = s.word_at(Point::new(l, 1), " \t").expect("a word");
    assert_eq!((start.col, end.col), (0, 4), "salud is one word");
}

// A mark is not what somebody searched for, and a letter wearing one is
// still that letter.
#[test]
fn searching_for_the_letter_finds_it_under_its_mark() {
    let mut s = Screen::new(20, 5, 100);
    with_marks(&mut s, "resume", &['\u{0301}']);

    let hits = s.search_all("resume", false);
    assert_eq!(hits.len(), 1, "the word is found through the accent");
}

#[test]
fn a_cluster_survives_the_window_changing_width() {
    let mut s = Screen::new(10, 4, 100);
    with_marks(&mut s, "a long enough line to be rewrapped", &['\u{0330}']);

    s.resize(40, 4);
    let found = (0..s.history_len() + s.rows() as u32)
        .flat_map(|l| (0..40).map(move |col| (l, col)))
        .find_map(|(l, col)| {
            s.grapheme_at(l, col)
                .filter(|g| g.is_cluster())
                .map(|g| g.to_string())
        });
    assert_eq!(found, Some("d\u{0330}".into()));
}

#[test]
fn a_cluster_written_over_is_gone() {
    let mut s = Screen::new(20, 5, 100);
    let l = row(&s);
    with_marks(&mut s, "x", &['\u{0330}']);
    s.carriage_return();
    write(&mut s, "y");

    assert_eq!(s.row_text(l), "y");
    assert_eq!(cell_text(&s, l, 0), "y");
}

// The table is bounded, and the bound is reached by compacting away what
// has scrolled off before refusing anything.
#[test]
fn a_pane_that_keeps_making_clusters_keeps_showing_the_right_ones() {
    let mut s = Screen::new(40, 10, 200);
    for i in 0..5_000 {
        write(&mut s, &format!("line {i} "));
        assert!(s.attach('\u{0330}'));
        write(&mut s, "\r\n");
    }

    let newest = s.history_len() + s.rows() as u32 - 2;
    let text = s.row_text(newest);
    assert!(
        text.contains('\u{0330}'),
        "the newest cluster is still there: {text:?}"
    );
    for l in 0..s.history_len() + s.rows() as u32 {
        let line = s.row_text(l);
        assert!(
            !line
                .chars()
                .any(|c| ('\u{F0000}'..='\u{FFFFD}').contains(&c)),
            "row {l} leaked a handle: {line:?}"
        );
    }
}

#[test]
fn a_pane_with_no_clusters_is_no_bigger_for_the_feature() {
    let plain = Screen::new(200, 50, 10_000);
    let mut used = Screen::new(200, 50, 10_000);
    let before = used.memory_bytes();
    assert_eq!(before, plain.memory_bytes());

    with_marks(&mut used, "a", &['\u{0330}']);
    let grew = used.memory_bytes() - before;
    assert!(
        (3..128).contains(&grew),
        "one cluster cost {grew} bytes: its own text and nothing per cell"
    );
}

#[test]
fn the_cluster_flag_is_not_an_attribute_the_pen_carries() {
    assert!(
        !Flags::SGR_MASK.contains(Flags::CLUSTER),
        "it says where a cell's text lives, not how it looks"
    );
}
