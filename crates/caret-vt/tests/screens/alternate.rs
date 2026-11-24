use crate::common::Vt;

#[test]
fn the_alternate_screen_is_separate_and_restores() {
    let mut vt = Vt::with_scrollback(6, 2, 10);
    vt.feed("main\r\n");

    vt.feed("\x1b[?1049h");
    assert!(vt.term.is_alt());
    assert_eq!(vt.rows(), vec!["", ""], "the alternate screen starts blank");
    vt.feed("alt");
    assert_eq!(vt.row(0), "alt");

    vt.feed("\x1b[?1049l");
    assert!(!vt.term.is_alt());
    assert_eq!(vt.row(0), "main", "the primary screen is untouched");
}

#[test]
fn the_alternate_screen_keeps_no_scrollback() {
    let mut vt = Vt::with_scrollback(6, 2, 100);
    vt.feed("\x1b[?1049h");
    for i in 0..10 {
        vt.feed(&format!("x{i}\r\n"));
    }
    assert_eq!(
        vt.term.screen().history_len(),
        0,
        "a pager's redraws must not fill history"
    );
}

#[test]
fn entering_the_alternate_screen_twice_is_harmless() {
    let mut vt = Vt::new(6, 2);
    vt.feed("\x1b[?1049h").feed("alt").feed("\x1b[?1049h");
    assert_eq!(vt.row(0), "alt", "content was not wiped a second time");
}

#[test]
fn resizing_keeps_both_screens_consistent() {
    let mut vt = Vt::with_scrollback(10, 4, 10);
    vt.feed("hello");
    vt.term.resize(6, 2);
    assert_eq!(vt.term.cols(), 6);
    assert_eq!(vt.term.rows(), 2);

    vt.feed("\x1b[?1049h");
    assert_eq!(vt.term.cols(), 6, "the alternate screen was resized too");
    assert_eq!(vt.term.rows(), 2);
}
