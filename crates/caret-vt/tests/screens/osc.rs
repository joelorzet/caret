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
