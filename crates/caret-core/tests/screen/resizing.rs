use crate::common::{viewport, write};
use caret_core::Screen;

#[test]
fn growing_the_viewport_reveals_history_and_the_cursor_follows() {
    let mut s = Screen::new(8, 2, 10);
    write(&mut s, "A\r\nB\r\nC\r\nD");
    assert_eq!(s.history_len(), 2);
    assert_eq!(s.cursor().y, 1);

    s.resize(8, 4);
    assert_eq!(viewport(&s), vec!["A", "B", "C", "D"]);
    assert_eq!(s.history_len(), 0);
    assert_eq!(s.cursor().y, 3, "cursor stays on the row holding D");

    s.resize(8, 2);
    assert_eq!(viewport(&s), vec!["C", "D"]);
    assert_eq!(s.cursor().y, 1);
}

#[test]
fn shrinking_columns_truncates_each_row() {
    let mut s = Screen::new(8, 2, 10);
    write(&mut s, "abcdef\r\nghijkl");
    s.resize(3, 2);
    assert_eq!(viewport(&s), vec!["abc", "ghi"]);
    assert_eq!(s.cols(), 3);
}

#[test]
fn growing_columns_pads_without_losing_content() {
    let mut s = Screen::new(4, 2, 10);
    write(&mut s, "ab\r\ncd");
    s.resize(10, 2);
    assert_eq!(viewport(&s), vec!["ab", "cd"]);
    assert_eq!(s.cols(), 10);
}

#[test]
fn resize_drops_the_oldest_rows_when_capacity_shrinks() {
    let mut s = Screen::new(8, 2, 10);
    for i in 0..8 {
        write(&mut s, &format!("L{i}\r\n"));
    }
    s.resize(8, 2);
    s.resize(8, 1);
    assert!(s.history_len() <= 10);
}

#[test]
fn resize_resets_the_scroll_region() {
    let mut s = Screen::new(8, 6, 0);
    s.set_region(2, 4);
    s.resize(8, 10);
    assert_eq!((s.region().top, s.region().bottom), (0, 9));
}
