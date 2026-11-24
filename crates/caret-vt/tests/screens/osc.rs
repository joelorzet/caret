use crate::common::Vt;
use caret_core::TerminalEvent;

#[test]
fn the_title_is_reported() {
    let mut vt = Vt::new(6, 1);
    vt.feed("\x1b]0;my title\x07");
    assert_eq!(vt.term.title(), "my title");
    let events: Vec<_> = vt.term.drain_events().collect();
    assert_eq!(events, vec![TerminalEvent::TitleChanged]);

    vt.feed("\x1b]2;another\x1b\\");
    assert_eq!(vt.term.title(), "another");
}

#[test]
fn an_unchanged_title_is_not_reported_again() {
    let mut vt = Vt::new(6, 1);
    vt.feed("\x1b]0;same\x07");
    let _ = vt.term.drain_events().count();
    vt.feed("\x1b]0;same\x07");
    assert_eq!(vt.term.drain_events().count(), 0);
}

// OSC 7 is how a shell reports its directory, which the explorer follows.
#[test]
fn the_working_directory_is_read_from_osc_7() {
    let mut vt = Vt::new(6, 1);
    vt.feed("\x1b]7;file://host/Users/me/code\x07");
    assert_eq!(vt.term.cwd(), Some("/Users/me/code"));
    let events: Vec<_> = vt.term.drain_events().collect();
    assert_eq!(events, vec![TerminalEvent::CwdChanged]);
}

#[test]
fn percent_escapes_in_the_directory_are_decoded() {
    let mut vt = Vt::new(6, 1);
    vt.feed("\x1b]7;file://host/tmp/my%20dir\x07");
    assert_eq!(vt.term.cwd(), Some("/tmp/my dir"));
}

#[test]
fn a_malformed_directory_url_is_ignored() {
    let mut vt = Vt::new(6, 1);
    vt.feed("\x1b]7;not-a-url\x07");
    assert_eq!(vt.term.cwd(), None);
}

#[test]
fn the_prompt_mark_is_reported() {
    let mut vt = Vt::new(6, 1);
    vt.feed("\x1b]133;A\x07");
    let events: Vec<_> = vt.term.drain_events().collect();
    assert_eq!(events, vec![TerminalEvent::PromptMarked]);
}

#[test]
fn an_unknown_osc_is_ignored() {
    let mut vt = Vt::new(6, 1);
    vt.feed("\x1b]99;whatever\x07ok");
    assert_eq!(vt.row(0), "ok");
}

// Yanking in an editor inside a pane asks the terminal to put the text on
// the clipboard. Ignoring it, as this did, means the yank goes nowhere.
#[test]
fn a_program_can_ask_for_text_to_go_on_the_clipboard() {
    let mut vt = Vt::new(20, 4);
    vt.feed("\x1b]52;c;aGVsbG8gZnJvbSB2aW0=\x07");
    assert_eq!(
        vt.term.drain_events().collect::<Vec<_>>(),
        vec![TerminalEvent::ClipboardWrite("hello from vim".into())]
    );
}

#[test]
fn the_target_it_names_does_not_matter_to_us() {
    for targets in ["c", "p", "s0", "", "cp"] {
        let mut vt = Vt::new(20, 4);
        vt.feed(&format!("\x1b]52;{targets};Zm9vYmFy\x07"));
        assert_eq!(
            vt.term.drain_events().collect::<Vec<_>>(),
            vec![TerminalEvent::ClipboardWrite("foobar".into())],
            "{targets}"
        );
    }
}

// A program that can read the clipboard can read what you copied out of a
// password manager. The request is never answered.
#[test]
fn a_request_to_read_the_clipboard_is_ignored() {
    let mut vt = Vt::new(20, 4);
    vt.feed("\x1b]52;c;?\x07");
    assert!(vt.term.drain_events().next().is_none());
}

#[test]
fn a_malformed_or_oversized_request_is_ignored() {
    let mut vt = Vt::new(20, 4);
    vt.feed("\x1b]52;c;not base64!\x07");
    assert!(vt.term.drain_events().next().is_none(), "not base64");

    let huge = "A".repeat((1 << 20) + 4);
    vt.feed(&format!("\x1b]52;c;{huge}\x07"));
    assert!(vt.term.drain_events().next().is_none(), "over the limit");

    vt.feed("\x1b]52;c\x07");
    assert!(vt.term.drain_events().next().is_none(), "no data at all");
}
