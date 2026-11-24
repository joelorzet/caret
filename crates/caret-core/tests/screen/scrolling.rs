use crate::common::{viewport, write};
use caret_core::Screen;

#[test]
fn full_screen_scroll_retires_rows_into_history() {
    let mut s = Screen::new(8, 2, 10);
    write(&mut s, "A\r\nB\r\nC");
    assert_eq!(s.history_len(), 1);
    assert_eq!(viewport(&s), vec!["B", "C"]);
    assert_eq!(s.row_text(0), "A", "A is now the oldest history row");
}

// Exercises slot recycling: once the scrollback is full the ring wraps, and a
// bug in the modular arithmetic shows up as scrambled or duplicated rows.
#[test]
fn ring_recycles_slots_once_scrollback_is_full() {
    let mut s = Screen::new(8, 2, 3); // capacity 5 rows
    for i in 0..10 {
        write(&mut s, &format!("L{i}\r\n"));
    }

    // 11 logical rows were produced (L0..L9 plus the blank the final newline
    // opened); only the newest 5 survive.
    assert_eq!(s.history_len(), 3);
    let rows: Vec<String> = (0..5).map(|l| s.row_text(l)).collect();
    assert_eq!(rows, vec!["L6", "L7", "L8", "L9", ""]);
    assert_eq!(viewport(&s), vec!["L9", ""]);
}

#[test]
fn steady_state_output_never_reallocates() {
    let mut s = Screen::new(80, 24, 500);
    let before = s.memory_bytes();
    for i in 0..5_000 {
        write(&mut s, &format!("line {i}\r\n"));
    }
    assert_eq!(s.memory_bytes(), before, "grid allocation is sized once");
    assert_eq!(s.history_len(), 500, "scrollback caps out");
}

#[test]
fn scroll_region_confines_scrolling() {
    let mut s = Screen::new(8, 5, 10);
    write(&mut s, "A\r\nB\r\nC\r\nD\r\nE");
    assert_eq!(s.history_len(), 0);

    s.set_region(1, 3);
    s.move_to(0, 3);
    s.line_feed();

    assert_eq!(viewport(&s), vec!["A", "C", "D", "", "E"]);
    assert_eq!(
        s.history_len(),
        0,
        "a partial region discards rather than saves"
    );
}

#[test]
fn inverted_region_resets_to_full_screen() {
    let mut s = Screen::new(8, 5, 0);
    s.set_region(4, 2);
    assert_eq!((s.region().top, s.region().bottom), (0, 4));
}

#[test]
fn reverse_line_feed_scrolls_down_at_the_region_top() {
    let mut s = Screen::new(8, 3, 10);
    write(&mut s, "A\r\nB\r\nC");
    s.move_to(0, 0);
    s.reverse_line_feed();
    assert_eq!(viewport(&s), vec!["", "A", "B"]);
}

#[test]
fn insert_and_delete_lines_stay_inside_the_region() {
    let mut s = Screen::new(8, 4, 0);
    write(&mut s, "A\r\nB\r\nC\r\nD");

    s.move_to(0, 1);
    s.insert_lines(1);
    assert_eq!(viewport(&s), vec!["A", "", "B", "C"]);

    s.move_to(0, 1);
    s.delete_lines(1);
    assert_eq!(viewport(&s), vec!["A", "B", "C", ""]);
}

#[test]
fn insert_lines_outside_the_region_is_ignored() {
    let mut s = Screen::new(8, 4, 0);
    write(&mut s, "A\r\nB\r\nC\r\nD");
    s.set_region(0, 1);
    s.move_to(0, 3);
    s.insert_lines(2);
    assert_eq!(viewport(&s), vec!["A", "B", "C", "D"]);
}
