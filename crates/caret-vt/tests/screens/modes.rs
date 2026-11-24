use crate::common::Vt;
use caret_core::Modes;

#[test]
fn modes_are_tracked() {
    let mut vt = Vt::new(6, 2);
    for (sequence, mode) in [
        ("\x1b[?1h", Modes::APP_CURSOR),
        ("\x1b[?2004h", Modes::BRACKETED_PASTE),
        ("\x1b[?1000h", Modes::MOUSE_CLICK),
        ("\x1b[?1002h", Modes::MOUSE_DRAG),
        ("\x1b[?1003h", Modes::MOUSE_MOTION),
        ("\x1b[?1006h", Modes::MOUSE_SGR),
        ("\x1b[?1004h", Modes::FOCUS_EVENTS),
        ("\x1b[?2026h", Modes::SYNC_UPDATE),
        ("\x1b[4h", Modes::INSERT),
        ("\x1b[20h", Modes::LINE_FEED_NL),
    ] {
        vt.feed(sequence);
        assert!(
            vt.term.modes().contains(mode),
            "{sequence} did not set the mode"
        );
    }

    vt.feed("\x1b[?1l\x1b[?1000l\x1b[4l");
    assert!(!vt.term.modes().contains(Modes::APP_CURSOR));
    assert!(!vt.term.modes().contains(Modes::MOUSE_CLICK));
    assert!(!vt.term.modes().contains(Modes::INSERT));
}

#[test]
fn several_modes_can_be_set_at_once() {
    let mut vt = Vt::new(6, 2);
    vt.feed("\x1b[?1000;1006h");
    assert!(vt.term.modes().contains(Modes::MOUSE_CLICK));
    assert!(vt.term.modes().contains(Modes::MOUSE_SGR));
}

#[test]
fn line_feed_newline_mode_adds_the_carriage_return() {
    let mut vt = Vt::new(6, 3);
    vt.feed("\x1b[20h").feed("abc\nx");
    assert_eq!(vt.row(1), "x");
}
