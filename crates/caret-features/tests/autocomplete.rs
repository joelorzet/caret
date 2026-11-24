use caret_features::{history, Autocomplete};

fn seeded() -> Autocomplete {
    Autocomplete::new(
        [
            "git status",
            "cargo build --release",
            "cargo test",
            "ls -la",
            "cargo build",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect(),
    )
}

#[test]
fn nothing_is_suggested_for_an_empty_input() {
    let mut auto = seeded();
    auto.set_input("");
    assert!(!auto.is_active());
    assert_eq!(auto.suggestion(), None);
    assert_eq!(auto.remainder(), None);
}

#[test]
fn a_prefix_suggests_the_most_recent_match_first() {
    let mut auto = seeded();
    auto.set_input("cargo ");
    assert!(auto.is_active());
    // "cargo build" was recorded last, so it is offered first.
    assert_eq!(auto.suggestion(), Some("cargo build"));
    assert_eq!(auto.remainder(), Some("build"));
}

// The remainder is what gets typed when the suggestion is accepted, so it has
// to be exactly the untyped tail.
#[test]
fn the_remainder_is_only_the_untyped_part() {
    let mut auto = seeded();
    auto.set_input("git s");
    assert_eq!(auto.suggestion(), Some("git status"));
    assert_eq!(auto.remainder(), Some("tatus"));
}

#[test]
fn matching_ignores_case_but_keeps_what_was_typed() {
    let mut auto = seeded();
    auto.set_input("CARGO T");
    assert_eq!(auto.suggestion(), Some("cargo test"));
}

#[test]
fn only_continuations_are_offered() {
    let mut auto = seeded();
    auto.set_input("cargo test");
    // An entry that is exactly what is typed has nothing left to complete.
    assert!(!auto.is_active(), "{:?}", auto.suggestion());

    auto.set_input("make");
    assert!(!auto.is_active(), "no entry starts with that");
}

#[test]
fn arrows_move_through_the_matches_and_wrap() {
    let mut auto = seeded();
    auto.set_input("cargo");
    let all: Vec<&str> = auto.matches().collect();
    assert_eq!(
        all,
        vec!["cargo build", "cargo test", "cargo build --release"]
    );

    assert!(auto.select_next());
    assert_eq!(auto.suggestion(), Some("cargo test"));
    assert!(auto.select_next());
    assert_eq!(auto.suggestion(), Some("cargo build --release"));
    assert!(auto.select_next());
    assert_eq!(auto.suggestion(), Some("cargo build"), "wraps round");

    assert!(auto.select_previous());
    assert_eq!(auto.suggestion(), Some("cargo build --release"));
}

#[test]
fn a_single_match_has_nowhere_to_move() {
    let mut auto = seeded();
    auto.set_input("ls");
    assert_eq!(auto.match_count(), 1);
    assert!(!auto.select_next());
    assert_eq!(auto.selected(), 0);
}

#[test]
fn editing_the_input_resets_the_selection() {
    let mut auto = seeded();
    auto.set_input("cargo");
    auto.select_next();
    assert_eq!(auto.selected(), 1);

    auto.set_input("cargo t");
    assert_eq!(auto.selected(), 0, "a new query starts at the top");
}

#[test]
fn an_unchanged_input_is_not_recomputed() {
    let mut auto = seeded();
    assert!(auto.set_input("cargo"));
    assert!(!auto.set_input("cargo"), "no change to report");
}

// Running a command again should promote it, not duplicate it.
#[test]
fn recording_promotes_rather_than_duplicates() {
    let mut auto = seeded();
    auto.record("cargo test");
    assert_eq!(auto.history().len(), 5, "still five entries");
    assert_eq!(
        auto.history().last().map(String::as_str),
        Some("cargo test")
    );

    auto.set_input("cargo ");
    assert_eq!(auto.suggestion(), Some("cargo test"), "now the most recent");
}

#[test]
fn blank_commands_are_not_recorded() {
    let mut auto = Autocomplete::new(Vec::new());
    auto.record("   ");
    auto.record("");
    assert!(auto.history().is_empty());
}

#[test]
fn recorded_commands_are_trimmed() {
    let mut auto = Autocomplete::new(Vec::new());
    auto.record("  ls -la  ");
    assert_eq!(auto.history(), ["ls -la"]);
}

#[test]
fn clearing_drops_the_suggestions_but_keeps_the_history() {
    let mut auto = seeded();
    auto.set_input("cargo");
    auto.clear();
    assert!(!auto.is_active());
    assert_eq!(auto.input(), "");
    assert_eq!(auto.history().len(), 5);
}

#[test]
fn leading_spaces_do_not_break_matching() {
    let mut auto = seeded();
    auto.set_input("   git");
    assert_eq!(auto.suggestion(), Some("git status"));
}

// --- the history file ------------------------------------------------------

fn temp(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("caret-hist-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir.join("history")
}

#[test]
fn history_round_trips_through_a_file() {
    let path = temp("roundtrip");
    history::append(&path, "cargo build").unwrap();
    history::append(&path, "git status").unwrap();

    let loaded = history::load(&path, 100);
    assert_eq!(loaded, ["cargo build", "git status"], "oldest first");
}

#[test]
fn a_missing_history_file_is_simply_empty() {
    assert!(history::load(std::path::Path::new("/nonexistent/caret/history"), 10).is_empty());
}

#[test]
fn only_the_newest_entries_are_kept() {
    let path = temp("limit");
    for i in 0..10 {
        history::append(&path, &format!("command {i}")).unwrap();
    }
    let loaded = history::load(&path, 3);
    assert_eq!(loaded, ["command 7", "command 8", "command 9"]);
}

#[test]
fn blank_and_multiline_commands_are_not_written() {
    let path = temp("blank");
    history::append(&path, "  ").unwrap();
    history::append(&path, "a\nb").unwrap();
    assert!(history::load(&path, 10).is_empty());
}
