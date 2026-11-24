mod common;
use common::Vt;

fn row(vt: &Vt, y: u16) -> String {
    vt.term
        .screen()
        .display_line(y)
        .iter()
        .map(|c| if c.ch == '\0' { ' ' } else { c.ch })
        .collect::<String>()
        .trim_end()
        .to_string()
}

/// The row the cursor is sitting on.
fn under_cursor(vt: &Vt) -> String {
    row(vt, vt.term.screen().cursor().y)
}

// Dragging the window to a display with a different pixel density resizes
// the grid, and the prompt was being thrown away: the window had been padded
// out with blank rows, and shrinking kept those and dropped what was written.
#[test]
fn shrinking_keeps_what_was_written_rather_than_the_blank_room_below_it() {
    let mut vt = Vt::new(80, 24);
    vt.feed("prompt> ");

    vt.term.resize(80, 12);
    assert_eq!(row(&vt, 0), "prompt>");
    assert_eq!(under_cursor(&vt), "prompt>", "the cursor is still on it");
}

#[test]
fn the_cursor_stays_on_its_line_through_any_sequence_of_resizes() {
    let mut vt = Vt::new(80, 24);
    vt.feed("one\r\ntwo\r\nthree> ");

    for (cols, rows) in [(80, 12), (80, 40), (60, 20), (100, 50), (80, 24)] {
        vt.term.resize(cols, rows);
        assert_eq!(under_cursor(&vt), "three>", "at {cols}x{rows}");
    }
}

// With scrollback, growing the window reveals what is above rather than
// adding blank space below, so the cursor moves down the screen with it.
#[test]
fn growing_reveals_scrollback_above_and_the_cursor_moves_with_it() {
    let mut vt = Vt::with_scrollback(80, 6, 100);
    for n in 0..12 {
        vt.feed(&format!("line {n}\r\n"));
    }
    vt.feed("last> ");

    let before = vt.term.screen().cursor().y;
    let top_before = row(&vt, 0);
    let history_before = vt.term.screen().history_len();

    vt.term.resize(80, 12);

    assert!(
        vt.term.screen().cursor().y > before,
        "the cursor moved down the screen: {before} then {}",
        vt.term.screen().cursor().y
    );
    assert_eq!(under_cursor(&vt), "last>", "and stayed on its own line");
    assert_ne!(row(&vt, 0), top_before, "the top of the screen went back");
    assert!(
        vt.term.screen().history_len() < history_before,
        "which is scrollback that came into view"
    );
}

#[test]
fn shrinking_pushes_the_top_into_scrollback_and_keeps_the_cursor_visible() {
    let mut vt = Vt::with_scrollback(80, 12, 100);
    for n in 0..10 {
        vt.feed(&format!("line {n}\r\n"));
    }
    vt.feed("last> ");

    vt.term.resize(80, 4);
    assert_eq!(under_cursor(&vt), "last>");
    assert!(
        vt.term.screen().cursor().y < 4,
        "the cursor is on screen, not past the bottom"
    );
    assert!(
        vt.term.screen().history_len() > 0,
        "what came off the top is in the scrollback"
    );
}

#[test]
fn a_narrower_window_keeps_the_text_that_still_fits() {
    let mut vt = Vt::new(80, 24);
    vt.feed("a line of text that is longer than forty columns wide");

    vt.term.resize(40, 24);
    assert_eq!(row(&vt, 0), "a line of text that is longer than forty");
}

#[test]
fn resizing_to_the_same_size_changes_nothing() {
    let mut vt = Vt::new(80, 24);
    vt.feed("prompt> ");
    let before = vt.term.screen().cursor();
    vt.term.resize(80, 24);
    assert_eq!(vt.term.screen().cursor(), before);
    assert_eq!(row(&vt, 0), "prompt>");
}

// One cell is the smallest a window can get to, and it must not panic.
#[test]
fn a_window_squeezed_to_nothing_still_holds_together() {
    let mut vt = Vt::new(80, 24);
    vt.feed("prompt> ");
    for (cols, rows) in [(1, 1), (0, 0), (1, 40), (80, 1)] {
        vt.term.resize(cols, rows);
        let cursor = vt.term.screen().cursor();
        assert!(cursor.y < vt.term.screen().rows());
        assert!(cursor.x < vt.term.screen().cols());
    }
}
