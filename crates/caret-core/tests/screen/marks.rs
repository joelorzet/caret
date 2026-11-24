use crate::common::write;
use caret_core::Screen;

/// Writes `count` lines, marking a prompt before each one.
fn with_prompts(s: &mut Screen, count: usize) {
    for i in 0..count {
        s.mark_prompt();
        write(s, &format!("prompt> command {i}\r\noutput of {i}\r\n"));
    }
}

#[test]
fn a_prompt_is_remembered_where_the_cursor_was() {
    let mut s = Screen::new(40, 10, 100);
    with_prompts(&mut s, 3);
    assert_eq!(s.prompt_count(), 3);

    // From the bottom, each jump goes back one command.
    let bottom = s.history_len() + s.rows() as u32 - 1;
    let first = s.prompt_above(bottom).expect("a prompt above");
    let second = s.prompt_above(first).expect("another above that");
    assert!(second < first);
    assert_eq!(s.row_text(first), "prompt> command 2");
    assert_eq!(s.row_text(second), "prompt> command 1");
}

#[test]
fn there_is_nothing_above_the_first_or_below_the_last() {
    let mut s = Screen::new(40, 10, 100);
    with_prompts(&mut s, 2);
    let first = s.prompt_above(1).or_else(|| s.prompt_below(0));
    assert!(first.is_some());
    assert!(s.prompt_above(0).is_none(), "nothing before the first row");

    let bottom = s.history_len() + s.rows() as u32 - 1;
    assert!(s.prompt_below(bottom).is_none(), "nothing after the last");
}

#[test]
fn the_same_row_marked_twice_is_one_mark() {
    let mut s = Screen::new(40, 10, 100);
    s.mark_prompt();
    s.mark_prompt();
    assert_eq!(s.prompt_count(), 1);
}

// A mark on a row that has scrolled out of the scrollback is gone, and
// must not point at whatever row has taken its number.
#[test]
fn a_mark_is_forgotten_when_its_row_leaves() {
    let mut s = Screen::new(20, 2, 4);
    s.mark_prompt();
    write(&mut s, "first\r\n");
    for i in 0..20 {
        write(&mut s, &format!("filler {i}\r\n"));
    }
    assert_eq!(s.prompt_count(), 0, "the marked row is long gone");
}

// Rewrapping renumbers every row, so a mark moves with the line it is on.
#[test]
fn a_mark_survives_a_change_of_width() {
    let mut s = Screen::new(40, 6, 100);
    s.mark_prompt();
    write(
        &mut s,
        "prompt> a command that is long enough to wrap later\r\n",
    );
    s.mark_prompt();
    write(&mut s, "prompt> second\r\n");
    assert_eq!(s.prompt_count(), 2);

    s.resize(16, 6);
    assert_eq!(s.prompt_count(), 2, "both marks are still here");

    let bottom = s.history_len() + s.rows() as u32 - 1;
    let last = s.prompt_above(bottom).expect("a prompt");
    assert_eq!(s.row_text(last), "prompt> second");
}
