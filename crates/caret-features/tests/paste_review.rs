use caret_features::review_paste;

// The ordinary case, and the one that must never be interrupted.
#[test]
fn one_line_of_plain_text_is_not_notable() {
    let review = review_paste("cargo test --workspace");
    assert_eq!(review.lines, 1);
    assert_eq!(review.chars, 22);
    assert!(!review.runs_on_arrival);
    assert!(review.controls.is_empty());
    assert!(!review.notable());
}

// The thing that turns a paste into a run: you did not press enter, the
// text did.
#[test]
fn a_trailing_newline_is_called_out() {
    let review = review_paste("rm -rf build\n");
    assert!(review.runs_on_arrival);
    assert!(review.notable());
    assert_eq!(review.lines, 1, "one line, and it ends");
}

#[test]
fn several_lines_are_counted() {
    let review = review_paste("one\ntwo\nthree");
    assert_eq!(review.lines, 3);
    assert!(!review.runs_on_arrival);
    assert!(review.notable());
}

// A snippet from a web page can carry anything. An escape sequence in a
// paste is somebody else's instruction to your terminal.
#[test]
fn control_characters_are_named_once_each() {
    let review = review_paste("echo hi\x1b[2Jthere\x1b]0;title\x07");
    assert_eq!(review.controls, ["escape", "bell"]);
    assert!(review.notable());
}

#[test]
fn tabs_and_newlines_are_not_treated_as_lurking_controls() {
    let review = review_paste("a\tb\nc\td");
    assert!(review.controls.is_empty());
}

#[test]
fn a_great_deal_of_text_is_notable_on_its_own() {
    let long = "x".repeat(2_001);
    assert!(review_paste(&long).notable());
    assert!(!review_paste(&"x".repeat(2_000)).notable());
}

#[test]
fn an_empty_paste_says_nothing_alarming() {
    let review = review_paste("");
    assert_eq!(review.chars, 0);
    assert!(!review.notable());
}
