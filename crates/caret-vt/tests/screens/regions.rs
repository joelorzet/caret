use crate::common::Vt;
use caret_core::Modes;

#[test]
fn a_scroll_region_confines_scrolling() {
    let mut vt = Vt::new(4, 5);
    vt.feed("aa\r\nbb\r\ncc\r\ndd\r\nee");

    // Rows 2..4 scroll; rows 1 and 5 stay put.
    vt.feed("\x1b[2;4r\x1b[4;1H\n");
    assert_eq!(vt.rows(), vec!["aa", "cc", "dd", "", "ee"]);
}

#[test]
fn setting_a_region_homes_the_cursor() {
    let mut vt = Vt::new(4, 5);
    vt.feed("\x1b[3;5H\x1b[2;4r");
    assert_eq!(vt.cursor(), (0, 0));
}

#[test]
fn a_region_reset_restores_the_whole_screen() {
    let mut vt = Vt::new(4, 4);
    vt.feed("\x1b[2;3r\x1b[r");
    assert_eq!(vt.term.screen().region().top, 0);
    assert_eq!(vt.term.screen().region().bottom, 3);
}

#[test]
fn scroll_up_and_down_commands() {
    let mut vt = Vt::new(3, 3);
    vt.feed("aa\r\nbb\r\ncc");

    vt.feed("\x1b[1S");
    assert_eq!(vt.rows(), vec!["bb", "cc", ""]);

    vt.feed("\x1b[1T");
    assert_eq!(vt.rows(), vec!["", "bb", "cc"]);
}

// Origin mode makes addressing relative to the region.
#[test]
fn origin_mode_shifts_addressing() {
    let mut vt = Vt::new(6, 6);
    vt.feed("\x1b[3;5r\x1b[?6h\x1b[1;1HX");
    assert_eq!(vt.row(2), "X", "row 1 means the top of the region");
    assert!(vt.term.modes().contains(Modes::ORIGIN));

    vt.feed("\x1b[?6l\x1b[1;1HY");
    assert_eq!(vt.row(0), "Y");
}

#[test]
fn output_fills_the_scrollback() {
    let mut vt = Vt::with_scrollback(8, 2, 10);
    for i in 0..6 {
        vt.feed(&format!("line{i}\r\n"));
    }
    assert_eq!(vt.term.screen().history_len(), 5);
    assert_eq!(vt.row(0), "line5");
}
