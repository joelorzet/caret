mod common;
use common::Vt;

/// What Claude Code writes when it starts, captured from its pseudoterminal.
///
/// It ends by homing, placing the cursor in its input box and showing it, so
/// a terminal that still has the cursor at the top of the screen has lost one
/// of those sequences somewhere earlier in the frame.
const STARTUP: &str = include_str!("fixtures/claude_startup.txt");

#[test]
fn a_programs_opening_frame_leaves_the_cursor_where_it_put_it() {
    let mut vt = Vt::new(44, 29);
    vt.feed(STARTUP);
    let cursor = vt.term.screen().cursor();
    assert!(vt.term.cursor_visible(), "it asked for the cursor back");
    assert_eq!(
        (cursor.y, cursor.x),
        (25, 3),
        "the frame ends with ESC[26;4H, which is row 26 column 4, one-based"
    );
}

// `CSI ? u` asks which keyboard protocol is in use. Run as `CSI u` it
// restored a cursor that was never saved.
#[test]
fn a_private_query_does_not_move_the_cursor() {
    for query in ["\u{1b}[?u", "\u{1b}[?s", "\u{1b}[?r", "\u{1b}[>u"] {
        let mut vt = Vt::new(20, 10);
        vt.feed("\u{1b}[5;7H");
        vt.feed(query);
        let cursor = vt.term.screen().cursor();
        assert_eq!((cursor.y, cursor.x), (4, 6), "{query:?} moved the cursor");
    }
}

#[test]
fn saving_and_restoring_the_cursor_still_works() {
    let mut vt = Vt::new(20, 10);
    vt.feed("\u{1b}[5;7H\u{1b}[s\u{1b}[1;1H\u{1b}[u");
    let cursor = vt.term.screen().cursor();
    assert_eq!((cursor.y, cursor.x), (4, 6));
}

// The private form of the cursor report has its own reply shape; answering
// in the plain form has the asker read it as a different sequence.
#[test]
fn the_private_cursor_report_replies_in_the_private_form() {
    let mut vt = Vt::new(20, 10);
    vt.feed("\u{1b}[3;5H\u{1b}[?6n");
    let reply = String::from_utf8(vt.term.take_reply()).expect("utf-8");
    assert_eq!(reply, "\u{1b}[?3;5;1R");

    let mut vt = Vt::new(20, 10);
    vt.feed("\u{1b}[3;5H\u{1b}[6n");
    let reply = String::from_utf8(vt.term.take_reply()).expect("utf-8");
    assert_eq!(reply, "\u{1b}[3;5R");
}
