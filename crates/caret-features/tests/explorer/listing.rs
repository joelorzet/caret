use crate::support::{names, tree};
use caret_features::{EntryKind, Explorer, LineKind, PanelKey, PanelOutcome};

#[test]
fn lists_directories_first_then_names_ignoring_case() {
    let explorer = Explorer::new(tree("sort"), false);
    assert_eq!(
        names(&explorer),
        vec!["sub", "apple.txt", "Banana.txt", "run.sh"]
    );
}

#[test]
fn hidden_entries_are_omitted_unless_asked_for() {
    let root = tree("hidden");
    assert!(!names(&Explorer::new(&root, false)).contains(&".hidden"));
    assert!(names(&Explorer::new(&root, true)).contains(&".hidden"));
}

#[test]
fn entry_kinds_are_classified() {
    let explorer = Explorer::new(tree("kinds"), false);
    let kind = |name: &str| {
        explorer
            .entries()
            .iter()
            .find(|e| e.name == name)
            .unwrap()
            .kind
    };
    assert_eq!(kind("sub"), EntryKind::Directory);
    assert_eq!(kind("apple.txt"), EntryKind::File);
    #[cfg(unix)]
    assert_eq!(kind("run.sh"), EntryKind::Executable);
}

#[test]
fn an_empty_directory_reports_no_entries() {
    let root = tree("empty").join("sub/deep");
    let mut explorer = Explorer::new(&root, false);
    assert!(explorer.entries().is_empty());
    assert_eq!(explorer.handle(PanelKey::Down), PanelOutcome::Ignored);
    assert_eq!(explorer.handle(PanelKey::Right), PanelOutcome::Ignored);

    let lines = explorer.lines();
    assert_eq!(lines[1].kind, LineKind::Hint);
    assert_eq!(lines[1].text, "(empty)");
}

#[test]
fn an_unreadable_directory_shows_the_reason() {
    let explorer = Explorer::new("/nonexistent/caret-explorer", false);
    let lines = explorer.lines();
    assert_eq!(lines[1].kind, LineKind::Error);
    assert!(!lines[1].text.is_empty());
}

#[test]
fn lines_mark_the_selection_and_the_kinds() {
    let mut explorer = Explorer::new(tree("lines"), false);
    explorer.set_height(10);
    let lines = explorer.lines();

    assert_eq!(lines[0].kind, LineKind::Header);
    assert!(
        lines[0].text.starts_with("caret-exp-lines"),
        "{:?}",
        lines[0].text
    );

    let selected: Vec<&str> = lines
        .iter()
        .filter(|l| l.selected)
        .map(|l| l.text.as_str())
        .collect();
    assert_eq!(selected, vec!["sub/"], "exactly one line is selected");

    assert!(lines
        .iter()
        .any(|l| l.text == "run.sh*" && l.kind == LineKind::Executable));
    assert!(lines
        .iter()
        .any(|l| l.text == "apple.txt" && l.kind == LineKind::File));
}

#[test]
fn dot_toggles_hidden_files_and_keeps_the_selection() {
    let mut explorer = Explorer::new(tree("toggle"), false);
    explorer.handle(PanelKey::Down);
    let before = explorer.selected().unwrap().name.clone();

    explorer.handle(PanelKey::Char('.'));
    assert!(explorer.show_hidden());
    assert!(names(&explorer).contains(&".hidden"));
    assert_eq!(
        explorer.selected().unwrap().name,
        before,
        "selection followed"
    );

    explorer.handle(PanelKey::Char('.'));
    assert!(!explorer.show_hidden());
}
