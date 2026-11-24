use crate::support::tree;
use caret_features::{Explorer, PanelKey, PanelOutcome};

#[test]
fn n_opens_a_pane_in_the_highlighted_directory() {
    let root = tree("openpane");
    let mut explorer = Explorer::new(&root, false);
    // "sub" is first in the listing.
    assert_eq!(
        explorer.handle(PanelKey::Char('n')),
        PanelOutcome::OpenPane(root.join("sub"))
    );
}

#[test]
fn shift_n_opens_a_tab_instead() {
    let root = tree("opentab");
    let mut explorer = Explorer::new(&root, false);
    assert_eq!(
        explorer.handle(PanelKey::Char('N')),
        PanelOutcome::OpenTab(root.join("sub"))
    );
}

// A file has no directory of its own, so the folder being browsed is used.
#[test]
fn opening_on_a_file_uses_the_folder_it_is_in() {
    let root = tree("openfile");
    let mut explorer = Explorer::new(&root, false);
    explorer.handle(PanelKey::Down); // apple.txt
    assert_eq!(
        explorer.handle(PanelKey::Char('n')),
        PanelOutcome::OpenPane(root.clone())
    );
}

#[test]
fn opening_does_not_move_the_explorer() {
    let root = tree("openstay");
    let mut explorer = Explorer::new(&root, false);
    explorer.handle(PanelKey::Char('n'));
    assert_eq!(explorer.root(), root);
}
