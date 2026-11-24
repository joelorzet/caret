use crate::common::{viewport, write};
use caret_core::Screen;

/// The text of every logical line, oldest first, with the wrapping undone.
///
/// Read from the cells rather than through `row_text`, which trims each
/// row: a space at the point where a line wrapped is part of the line.
fn lines(s: &Screen) -> Vec<String> {
    let total = s.history_len() + s.rows() as u32;
    let mut out = Vec::new();
    let mut row = 0;
    while row < total {
        let mut text = String::new();
        loop {
            for cell in s.logical_line(row) {
                text.push(if cell.ch == '\0' { ' ' } else { cell.ch });
            }
            if s.wrapped_row(row) && row + 1 < total {
                row += 1;
            } else {
                break;
            }
        }
        out.push(text.trim_end().to_string());
        row += 1;
    }
    while out.last().is_some_and(|line| line.is_empty()) {
        out.pop();
    }
    out
}

// The whole point: a line that no longer fits is laid out again, not cut.
#[test]
fn a_narrower_window_rewraps_rather_than_truncating() {
    let mut s = Screen::new(20, 6, 100);
    write(&mut s, "the quick brown fox jumps over the lazy dog");

    s.resize(10, 6);
    assert_eq!(
        lines(&s),
        ["the quick brown fox jumps over the lazy dog"],
        "the line is still every word it was"
    );
}

#[test]
fn widening_puts_a_wrapped_line_back_together() {
    let mut s = Screen::new(10, 6, 100);
    write(&mut s, "the quick brown fox jumps over the lazy dog");
    assert!(s.wrapped_row(0), "it wrapped at ten columns");

    // Forty-three characters, so it takes two rows at forty and one at
    // sixty.
    s.resize(40, 6);
    assert!(s.wrapped_row(0), "still two rows at forty");

    s.resize(60, 6);
    assert_eq!(lines(&s), ["the quick brown fox jumps over the lazy dog"]);
    assert!(!s.wrapped_row(0), "and one row at sixty");
}

#[test]
fn a_round_trip_through_several_widths_changes_nothing() {
    let written = "the quick brown fox jumps over the lazy dog";
    let mut s = Screen::new(50, 8, 100);
    write(&mut s, written);

    for width in [17, 9, 80, 33, 50] {
        s.resize(width, 8);
    }
    assert_eq!(lines(&s), [written]);
}

#[test]
fn separate_lines_stay_separate_however_often_it_is_resized() {
    let mut s = Screen::new(30, 8, 100);
    write(
        &mut s,
        "alpha beta gamma delta\r\nsecond line here\r\nthird",
    );

    s.resize(8, 8);
    s.resize(60, 8);
    assert_eq!(
        lines(&s),
        ["alpha beta gamma delta", "second line here", "third"]
    );
}

// Where the cursor is matters more than which row it is on: the shell
// redraws its prompt there.
#[test]
fn the_cursor_stays_on_the_character_it_was_on() {
    let mut s = Screen::new(20, 5, 100);
    write(&mut s, "hello");
    assert_eq!((s.cursor().y, s.cursor().x), (0, 5));

    s.resize(4, 5);
    // "hell" / "o", so the cursor is just after the o on the second row.
    assert_eq!(s.cursor().x, 1);
    assert_eq!(s.row_text(s.history_len() + s.cursor().y as u32), "o");

    s.resize(20, 5);
    assert_eq!((s.cursor().y, s.cursor().x), (0, 5));
}

#[test]
fn scrollback_is_rewrapped_too() {
    let mut s = Screen::new(20, 3, 100);
    for i in 0..6 {
        write(&mut s, &format!("line {i} with a few words in it\r\n"));
    }
    s.resize(12, 3);

    let all = lines(&s);
    assert_eq!(all.len(), 6, "{all:?}");
    for (i, line) in all.iter().enumerate() {
        assert_eq!(line, &format!("line {i} with a few words in it"));
    }
}

// A wide character is two cells and cannot be split down the middle.
#[test]
fn a_wide_character_is_never_broken_across_the_wrap() {
    let mut s = Screen::new(6, 4, 100);
    write(&mut s, "ab漢字cd");

    s.resize(5, 4);
    let text = lines(&s);
    assert_eq!(text, ["ab漢字cd"], "{text:?}");
}

// The alternate screen belongs to a program that redraws on resize;
// rewrapping what it drew would only scramble it.
#[test]
fn the_alternate_screen_is_not_rewrapped() {
    let mut s = Screen::alternate(20, 4);
    write(&mut s, "a line that is longer than ten columns");
    s.resize(10, 4);
    assert_eq!(s.cols(), 10);
    assert_eq!(viewport(&s).len(), 4);
}
