use crate::common::write;
use caret_core::Screen;

/// What the grid would take if it were sized for the whole scrollback at
/// birth: `(rows + scrollback) * cols` cells of sixteen bytes, plus the two
/// per-row side tables.
fn sized_for_everything(cols: u32, rows: u32, scrollback: u32) -> usize {
    let cap = (rows + scrollback) as usize;
    cap * cols as usize * 16 + cap * 4 + cap * 4
}

// A terminal with ten thousand lines of scrollback configured is not a
// terminal with ten thousand lines in it, and the difference is 32MB a pane.
#[test]
fn a_pane_does_not_pay_for_scrollback_it_has_not_filled() {
    let s = Screen::new(200, 50, 10_000);
    let everything = sized_for_everything(200, 50, 10_000);
    assert!(
        s.memory_bytes() * 8 < everything,
        "a fresh pane took {} of a possible {everything}",
        s.memory_bytes()
    );
}

#[test]
fn history_grows_as_it_fills_and_stops_at_the_limit() {
    let mut s = Screen::new(80, 24, 2_000);
    let fresh = s.memory_bytes();

    for i in 0..2_100 {
        write(&mut s, &format!("line {i}\r\n"));
    }
    let full = s.memory_bytes();
    assert!(full > fresh, "the buffer grew to hold the history");
    assert_eq!(s.history_len(), 2_000, "and stopped at the limit");

    // Past the limit it is a ring again: rows recycle in place and the
    // allocation never moves.
    for i in 2_100..7_000 {
        write(&mut s, &format!("line {i}\r\n"));
    }
    assert_eq!(s.memory_bytes(), full, "steady state allocates nothing");
    assert_eq!(s.history_len(), 2_000);
}

// Growing copies every row into a fresh allocation, which is exactly where a
// ring with an indirection table gets its order wrong.
#[test]
fn growing_keeps_every_line_in_order() {
    let mut s = Screen::new(8, 2, 4_000);
    for i in 0..3_000 {
        write(&mut s, &format!("L{i}\r\n"));
    }

    // 3,000 lines and the blank row the last newline opened, less the two
    // on screen.
    assert_eq!(s.history_len(), 2_999);
    for l in 0..2_999u32 {
        assert_eq!(s.row_text(l), format!("L{l}"), "logical row {l}");
    }
}

// The oldest rows still go first once the limit is reached, whatever the
// buffer did on the way there.
#[test]
fn the_limit_still_drops_the_oldest_lines() {
    let mut s = Screen::new(8, 2, 600);
    for i in 0..1_000 {
        write(&mut s, &format!("L{i}\r\n"));
    }

    assert_eq!(s.history_len(), 600);
    assert_eq!(s.row_text(0), "L399", "the oldest line still held");
    assert_eq!(s.row_text(599), "L998");
}

// A resize rebuilds the grid, which is the obvious place to quietly allocate
// the full scrollback after all.
#[test]
fn a_resize_keeps_the_room_the_pane_has_earned_and_no_more() {
    let mut s = Screen::new(80, 24, 10_000);
    let before = s.memory_bytes();
    s.resize(120, 40);
    let everything = sized_for_everything(120, 40, 10_000);
    assert!(
        s.memory_bytes() * 8 < everything,
        "a resize took {} of a possible {everything}",
        s.memory_bytes()
    );

    // Wider cells are more bytes a row, so this only has to stay in the same
    // country as it was.
    assert!(s.memory_bytes() < before * 3);
}
