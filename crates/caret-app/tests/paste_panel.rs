use caret_app::paste_notes;
use caret_features::review_paste;

// The summary is what somebody reads before deciding, so it says the two
// things that matter: how much, and what is lurking in it.
#[test]
fn the_summary_says_how_much_and_what_is_in_it() {
    let plain = paste_notes(&review_paste("one\ntwo")).join(" | ");
    assert!(plain.contains("2 line(s)"), "{plain}");
    assert!(plain.contains("7 character(s)"), "{plain}");
    assert!(!plain.contains("runs on arrival"), "{plain}");
}

#[test]
fn a_trailing_newline_is_spelled_out_rather_than_implied() {
    let summary = paste_notes(&review_paste("rm -rf build\n")).join(" | ");
    assert!(summary.contains("runs on arrival"), "{summary}");
}

#[test]
fn control_characters_are_named_in_the_summary() {
    let summary = paste_notes(&review_paste("hi\x1b[2Jthere")).join(" | ");
    assert!(summary.contains("escape"), "{summary}");
}
