use caret_app::{notice_for, Notice};
use caret_config::FinishNotice;
use std::time::Duration;

const AFTER: Duration = Duration::from_secs(30);
const LONG: Option<Duration> = Some(Duration::from_secs(45));
const SHORT: Option<Duration> = Some(Duration::from_secs(2));

// You watched it happen. Saying so is noise.
#[test]
fn the_pane_you_are_watching_never_announces_itself() {
    assert_eq!(
        notice_for(FinishNotice::System, AFTER, LONG, true, true),
        Notice::Nothing
    );
}

#[test]
fn a_long_command_elsewhere_reaches_the_status_line() {
    assert_eq!(
        notice_for(FinishNotice::Status, AFTER, LONG, false, true),
        Notice::Status
    );
}

// Behind another application, the status line reaches nobody.
#[test]
fn a_window_without_the_keyboard_gets_a_notification_if_allowed() {
    assert_eq!(
        notice_for(FinishNotice::System, AFTER, LONG, false, false),
        Notice::System
    );
    assert_eq!(
        notice_for(FinishNotice::Status, AFTER, LONG, false, false),
        Notice::Status,
        "but only if allowed"
    );
}

#[test]
fn a_short_command_says_nothing_however_it_ended() {
    for how in [FinishNotice::Status, FinishNotice::System] {
        assert_eq!(notice_for(how, AFTER, SHORT, false, false), Notice::Nothing);
        assert_eq!(notice_for(how, AFTER, None, false, false), Notice::Nothing);
    }
}

#[test]
fn turning_it_off_turns_it_off() {
    assert_eq!(
        notice_for(FinishNotice::Off, AFTER, LONG, false, false),
        Notice::Nothing
    );
}

// Exactly at the threshold counts: a thirty second threshold means thirty.
#[test]
fn the_threshold_is_inclusive() {
    let exactly = Some(AFTER);
    assert_eq!(
        notice_for(FinishNotice::Status, AFTER, exactly, false, true),
        Notice::Status
    );
}
