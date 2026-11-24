use crate::support::{names, tree};
use caret_features::{Explorer, PanelKey, PanelOutcome};
use std::path::PathBuf;

// Right goes in, left comes back out.
#[test]
fn right_enters_a_directory_and_left_returns() {
    let root = tree("nav");
    let mut explorer = Explorer::new(&root, false);

    assert_eq!(explorer.handle(PanelKey::Right), PanelOutcome::Redraw);
    assert_eq!(explorer.root(), root.join("sub"));
    assert_eq!(names(&explorer), vec!["deep"]);

    assert_eq!(explorer.handle(PanelKey::Left), PanelOutcome::Redraw);
    assert_eq!(explorer.root(), root);
}

// Walking back out should land on the directory you just left, not at the top.
#[test]
fn leaving_a_directory_selects_the_one_you_came_from() {
    let root = tree("comeback");
    let mut explorer = Explorer::new(&root, false);
    explorer.handle(PanelKey::Right);
    explorer.handle(PanelKey::Left);
    assert_eq!(explorer.selected().unwrap().name, "sub");
}

#[test]
fn right_on_a_file_hands_its_path_back() {
    let root = tree("insert");
    let mut explorer = Explorer::new(&root, false);
    explorer.handle(PanelKey::Down);

    let outcome = explorer.handle(PanelKey::Right);
    assert_eq!(outcome, PanelOutcome::Insert(root.join("apple.txt")));
    assert_eq!(explorer.root(), root, "the directory did not change");
}

#[test]
fn enter_behaves_like_right() {
    let root = tree("enter");
    let mut explorer = Explorer::new(&root, false);
    explorer.handle(PanelKey::Enter);
    assert_eq!(explorer.root(), root.join("sub"));
}

#[test]
fn left_at_the_filesystem_root_does_nothing() {
    let mut explorer = Explorer::new("/", false);
    assert_eq!(explorer.handle(PanelKey::Left), PanelOutcome::Ignored);
    assert_eq!(explorer.root(), PathBuf::from("/"));
}

#[test]
fn set_root_jumps_anywhere() {
    let root = tree("jump");
    let mut explorer = Explorer::new(&root, false);
    explorer.set_root(root.join("sub/deep"));
    assert_eq!(explorer.root(), root.join("sub/deep"));
    assert!(explorer.entries().is_empty());
}
