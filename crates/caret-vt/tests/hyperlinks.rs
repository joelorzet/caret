#[path = "common/mod.rs"]
mod common;

use common::Vt;

/// How a program marks a link: open with the target, print, close with none.
fn mark(target: &str, text: &str) -> String {
    format!("\x1b]8;;{target}\x07{text}\x1b]8;;\x07")
}

fn link_on_first_row(vt: &Vt, col: u16) -> Option<(usize, usize, String)> {
    let row = vt.term.screen().history_len();
    vt.term
        .screen()
        .marked_link(row, col)
        .map(|(start, end, target)| (start, end, target.to_owned()))
}

#[test]
fn a_program_can_say_that_text_is_a_link() {
    let mut vt = Vt::new(40, 5);
    vt.feed(&mark("https://example.com/build/7", "build 7"));

    assert_eq!(vt.row(0), "build 7");
    assert_eq!(
        link_on_first_row(&vt, 3),
        Some((0, 7, "https://example.com/build/7".into()))
    );
}

#[test]
fn closing_the_link_stops_it_claiming_what_comes_next() {
    let mut vt = Vt::new(40, 5);
    vt.feed(&mark("https://example.com/a", "link"));
    vt.feed(" plain");

    assert_eq!(vt.row(0), "link plain");
    assert!(link_on_first_row(&vt, 6).is_none());
}

// Semicolons separate the parts of the escape and are also legal in a URL,
// so the target has to be put back together.
#[test]
fn a_target_with_semicolons_in_it_arrives_whole() {
    let mut vt = Vt::new(40, 5);
    vt.feed("\x1b]8;;https://example.com/a;b;c\x07two\x1b]8;;\x07");

    assert_eq!(
        link_on_first_row(&vt, 0).map(|l| l.2),
        Some("https://example.com/a;b;c".into())
    );
}

// The id is for stitching a link split across lines back together; caret
// carries the target on the cells instead, so it is read and ignored.
#[test]
fn the_parameters_before_the_target_are_not_part_of_it() {
    let mut vt = Vt::new(40, 5);
    vt.feed("\x1b]8;id=42;https://example.com/z\x07text\x1b]8;;\x07");

    assert_eq!(
        link_on_first_row(&vt, 1).map(|l| l.2),
        Some("https://example.com/z".into())
    );
}

#[test]
fn an_escape_with_nothing_after_it_is_harmless() {
    let mut vt = Vt::new(40, 5);
    vt.feed("\x1b]8\x07plain\x1b]8;\x07more");

    assert_eq!(vt.row(0), "plainmore");
    assert!(link_on_first_row(&vt, 0).is_none());
}

#[test]
fn a_program_that_marks_nothing_has_no_links() {
    let mut vt = Vt::new(40, 5);
    vt.feed("https://example.com/plain-text");

    assert!(link_on_first_row(&vt, 4).is_none());
}
