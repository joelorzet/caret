#[path = "common/mod.rs"]
mod common;

use common::Vt;

/// What one cell of the first row shows, however many characters that is.
fn cell(vt: &Vt, col: u16) -> String {
    let row = vt.term.screen().history_len();
    vt.term
        .screen()
        .grapheme_at(row, col)
        .map(|g| g.to_string())
        .unwrap_or_default()
}

fn cursor_x(vt: &Vt) -> u16 {
    vt.cursor().0
}

#[test]
fn an_accent_after_its_letter_becomes_the_letter_with_the_accent() {
    let mut vt = Vt::new(20, 3);
    vt.feed("cafe\u{301}");

    assert_eq!(vt.row(0), "café", "composed into one character");
    assert_eq!(cell(&vt, 3), "é");
    assert_eq!(cursor_x(&vt), 4, "the accent took no room of its own");
}

#[test]
fn a_mark_with_no_precomposed_form_stays_beside_its_letter_in_one_cell() {
    let mut vt = Vt::new(20, 3);
    // A combining tilde below has no precomposed form with 'q'.
    vt.feed("q\u{0330}x");

    assert_eq!(cell(&vt, 0), "q\u{0330}", "one cell, two characters");
    assert_eq!(
        cell(&vt, 1),
        "x",
        "and the next letter is where it should be"
    );
    assert_eq!(cursor_x(&vt), 2);
}

#[test]
fn several_marks_all_land_on_the_same_letter() {
    let mut vt = Vt::new(20, 3);
    vt.feed("a\u{0330}\u{0331}\u{0332}");

    assert_eq!(cell(&vt, 0), "a\u{0330}\u{0331}\u{0332}");
    assert_eq!(cursor_x(&vt), 1);
}

#[test]
fn a_mark_with_nothing_before_it_is_dropped() {
    let mut vt = Vt::new(20, 3);
    vt.feed("\u{0301}abc");

    assert_eq!(vt.row(0), "abc");
    assert_eq!(cursor_x(&vt), 3);
}

#[test]
fn an_emoji_with_a_skin_tone_is_one_glyph_in_one_place() {
    let mut vt = Vt::new(20, 3);
    vt.feed("\u{1F44B}\u{1F3FD}!");

    assert_eq!(cell(&vt, 0), "\u{1F44B}\u{1F3FD}");
    // The wave is wide, so its own two columns, and then the mark.
    assert_eq!(cell(&vt, 2), "!");
    assert_eq!(cursor_x(&vt), 3);
}

#[test]
fn an_emoji_joined_to_another_is_one_cell() {
    let mut vt = Vt::new(20, 3);
    vt.feed("\u{1F468}\u{200D}\u{1F4BB}end");

    assert_eq!(cell(&vt, 0), "\u{1F468}\u{200D}\u{1F4BB}");
    assert_eq!(vt.row(0), "\u{1F468}\u{200D}\u{1F4BB}end");
    assert_eq!(cursor_x(&vt), 5, "two columns for the emoji, three letters");
}

#[test]
fn a_flag_is_its_pair_of_letters_and_the_next_flag_is_its_own() {
    let mut vt = Vt::new(20, 3);
    vt.feed("\u{1F1EA}\u{1F1F8}\u{1F1EB}\u{1F1F7}");

    assert_eq!(cell(&vt, 0), "\u{1F1EA}\u{1F1F8}", "one flag");
    let second = (1..6).find_map(|col| {
        let text = cell(&vt, col);
        (text.chars().count() == 2).then_some(text)
    });
    assert_eq!(
        second,
        Some("\u{1F1EB}\u{1F1F7}".to_string()),
        "and the second pair is a flag of its own"
    );
}

#[test]
fn a_variation_selector_belongs_to_the_character_before_it() {
    let mut vt = Vt::new(20, 3);
    vt.feed("\u{2764}\u{FE0F}x");

    assert_eq!(cell(&vt, 0), "\u{2764}\u{FE0F}");
    assert!(vt.row(0).ends_with('x'));
}

#[test]
fn text_with_no_marks_in_it_is_untouched() {
    let mut vt = Vt::new(20, 3);
    vt.feed("plain ascii");

    assert_eq!(vt.row(0), "plain ascii");
    assert_eq!(cursor_x(&vt), 11);
}

// The whole point of composing on the way in: what arrives decomposed is
// stored as the character somebody would actually type to look for it.
#[test]
fn searching_for_the_composed_word_finds_what_arrived_decomposed() {
    let mut vt = Vt::new(20, 3);
    vt.feed("re\u{0301}sume\u{0301} of the run");

    let hits = vt.term.screen().search_all("résumé", false);
    assert_eq!(hits.len(), 1, "found through the accents");
}

#[test]
fn a_cluster_counts_as_one_column_for_what_follows_it() {
    let mut vt = Vt::new(8, 3);
    vt.feed("q\u{0330}q\u{0330}q\u{0330}q\u{0330}q\u{0330}q\u{0330}q\u{0330}q\u{0330}");

    assert_eq!(cursor_x(&vt), 7, "eight cells wide, so the last column");
    assert_eq!(
        vt.row(0).chars().filter(|c| *c == 'q').count(),
        8,
        "all eight letters fit on the row"
    );
}
