use caret_update::CheckState;

const DAY: u64 = 24 * 3_600;

#[test]
fn a_check_is_due_once_the_interval_has_passed() {
    let state = CheckState {
        checked_at: 1_000_000,
        seen: "v0.1.0".into(),
    };
    assert!(!state.due(1_000_000 + DAY - 1, 24));
    assert!(state.due(1_000_000 + DAY, 24));
    assert!(state.due(1_000_000 + 2 * DAY, 24));
}

#[test]
fn a_state_that_has_never_been_written_is_due_at_once() {
    assert!(CheckState::default().due(1_000_000, 24));
}

// Clocks go backwards. A laptop that wakes up in the wrong decade should
// ask again, not never ask again.
#[test]
fn a_clock_that_went_backwards_is_due_rather_than_never() {
    let state = CheckState {
        checked_at: 2_000_000,
        seen: String::new(),
    };
    assert!(state.due(1_000, 24));
}

#[test]
fn the_file_round_trips_and_survives_nonsense() {
    let state = CheckState {
        checked_at: 1_234_567,
        seen: "v0.9.1".into(),
    };
    assert_eq!(CheckState::parse(&state.render()), state);

    let salvaged = CheckState::parse("checked_at = not-a-number\nseen = v1.0.0\nrubbish\n");
    assert_eq!(salvaged.checked_at, 0, "an unreadable time is no time");
    assert_eq!(salvaged.seen, "v1.0.0");
    assert_eq!(CheckState::parse(""), CheckState::default());
}

#[test]
fn the_state_is_written_beside_the_configuration_and_read_back() {
    let dir = std::env::temp_dir().join(format!("caret-update-state-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);

    let state = CheckState {
        checked_at: 42,
        seen: "v0.1.1".into(),
    };
    state.write(&dir);
    assert_eq!(CheckState::read(&dir), state);
    assert!(caret_update::state_path(&dir).ends_with("update-check"));

    // A directory that cannot be written is not a reason to fail to start.
    CheckState::default().write(std::path::Path::new("/caret-nowhere"));

    let _ = std::fs::remove_dir_all(&dir);
}
