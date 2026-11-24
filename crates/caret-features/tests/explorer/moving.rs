use crate::support::tree;
use caret_features::{Explorer, PanelKey, PanelOutcome};

#[test]
fn down_and_up_move_the_selection_and_stop_at_the_ends() {
    let mut explorer = Explorer::new(tree("move"), false);
    assert_eq!(explorer.selected().unwrap().name, "sub");

    explorer.handle(PanelKey::Down);
    assert_eq!(explorer.selected().unwrap().name, "apple.txt");

    for _ in 0..10 {
        explorer.handle(PanelKey::Down);
    }
    assert_eq!(
        explorer.selected().unwrap().name,
        "run.sh",
        "stops at the last"
    );

    for _ in 0..10 {
        explorer.handle(PanelKey::Up);
    }
    assert_eq!(
        explorer.selected().unwrap().name,
        "sub",
        "stops at the first"
    );
}

#[test]
fn home_and_end_jump_to_the_ends() {
    let mut explorer = Explorer::new(tree("ends"), false);
    explorer.handle(PanelKey::End);
    assert_eq!(explorer.selected().unwrap().name, "run.sh");
    explorer.handle(PanelKey::Home);
    assert_eq!(explorer.selected().unwrap().name, "sub");
}

#[test]
fn the_selection_scrolls_inside_the_panel() {
    let mut explorer = Explorer::new(tree("scroll"), false);
    explorer.set_height(2);

    // Four entries, two visible: moving to the last must bring it into view.
    explorer.handle(PanelKey::End);
    let rows = explorer.lines();
    let visible: Vec<&str> = rows.iter().skip(1).map(|l| l.text.as_str()).collect();
    assert_eq!(visible.len(), 2);
    assert!(visible.contains(&"run.sh*"), "{visible:?}");

    explorer.handle(PanelKey::Home);
    let visible: Vec<String> = explorer
        .lines()
        .iter()
        .skip(1)
        .map(|l| l.text.clone())
        .collect();
    assert_eq!(visible, vec!["sub/".to_string(), "apple.txt".to_string()]);
}

#[test]
fn paging_moves_by_the_panel_height() {
    let mut explorer = Explorer::new(tree("page"), false);
    explorer.set_height(2);
    explorer.handle(PanelKey::PageDown);
    assert_eq!(explorer.selected().unwrap().name, "Banana.txt");
    explorer.handle(PanelKey::PageUp);
    assert_eq!(explorer.selected().unwrap().name, "sub");
}

#[test]
fn unknown_characters_are_left_alone() {
    let mut explorer = Explorer::new(tree("unknown"), false);
    assert_eq!(explorer.handle(PanelKey::Char('z')), PanelOutcome::Ignored);
}

#[test]
fn escape_gives_focus_back() {
    let mut explorer = Explorer::new(tree("esc"), false);
    assert_eq!(explorer.handle(PanelKey::Escape), PanelOutcome::Dismiss);
}
