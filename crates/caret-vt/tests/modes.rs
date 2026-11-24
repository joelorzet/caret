use caret_vt::{AnsiMode, DecMode};

mod common;
use common::Vt;

// The numbers belong to the protocol, so they are written down once. A test
// that repeated them here would only prove the table matches itself; these
// check the ones whose meaning is easy to transpose.
#[test]
fn the_modes_programs_actually_send_are_recognised() {
    assert_eq!(DecMode::from_code(25), Some(DecMode::CursorVisible));
    assert_eq!(DecMode::from_code(1049), Some(DecMode::AltScreenSaveClear));
    assert_eq!(DecMode::from_code(1048), Some(DecMode::SaveCursor));
    assert_eq!(DecMode::from_code(1006), Some(DecMode::MouseSgr));
    assert_eq!(DecMode::from_code(2026), Some(DecMode::SynchronisedUpdate));
    assert_eq!(AnsiMode::from_code(4), Some(AnsiMode::Insert));
    assert_eq!(AnsiMode::from_code(20), Some(AnsiMode::LineFeedNewLine));
}

#[test]
fn a_mode_we_do_not_implement_is_not_mistaken_for_one_we_do() {
    for code in [0, 2, 3, 8, 9, 1005, 1015, 9999] {
        assert_eq!(DecMode::from_code(code), None, "{code}");
    }
    for code in [0, 1, 2, 3, 5, 12, 25] {
        assert_eq!(AnsiMode::from_code(code), None, "{code}");
    }
}

// 1048 saves the cursor on its own; 1049 saves it and switches screens. They
// are one digit apart and do different things.
#[test]
fn the_two_cursor_saving_modes_stay_apart() {
    let mut vt = Vt::new(20, 10);
    vt.feed("\u{1b}[5;7H\u{1b}[?1048h\u{1b}[1;1H\u{1b}[?1048l");
    let cursor = vt.term.screen().cursor();
    assert_eq!((cursor.y, cursor.x), (4, 6), "1048 restores the cursor");

    let mut vt = Vt::new(20, 10);
    vt.feed("hello\u{1b}[?1049h");
    assert!(vt.term.is_alt(), "1049 switches screens");
}
