use crate::support::{write, CODE};
use caret_features::{PanelKey, PanelOutcome, Preview};

#[test]
fn without_a_selection_the_cursor_line_is_what_copies() {
    let mut preview = Preview::open(&write("one", "main.rs", CODE));
    preview.handle(PanelKey::Down);
    assert_eq!(preview.selection(), (1, 1));
    assert_eq!(preview.selected_text(), "    let x = 1;");
}

#[test]
fn v_starts_a_selection_that_grows_with_the_cursor() {
    let mut preview = Preview::open(&write("select", "main.rs", CODE));
    preview.set_height(10);

    preview.handle(PanelKey::Char('v'));
    assert!(preview.is_selecting());
    preview.handle(PanelKey::Down);
    preview.handle(PanelKey::Down);

    assert_eq!(preview.selection(), (0, 2));
    assert_eq!(
        preview.selected_text(),
        "pub fn main() {\n    let x = 1;\n    // a note"
    );
}

#[test]
fn a_selection_made_upwards_still_reads_downwards() {
    let mut preview = Preview::open(&write("up", "main.rs", CODE));
    preview.handle(PanelKey::End);
    preview.handle(PanelKey::Char('v'));
    preview.handle(PanelKey::Up);
    preview.handle(PanelKey::Up);

    assert_eq!(preview.selection(), (2, 4));
}

#[test]
fn y_hands_back_the_text_and_ends_the_selection() {
    let mut preview = Preview::open(&write("copy", "main.rs", CODE));
    preview.handle(PanelKey::Char('v'));
    preview.handle(PanelKey::Down);

    match preview.handle(PanelKey::Char('y')) {
        PanelOutcome::Copy(text) => {
            assert_eq!(text, "pub fn main() {\n    let x = 1;");
        }
        other => panic!("expected a copy, got {other:?}"),
    }
    assert!(!preview.is_selecting(), "the selection is finished with");
}

#[test]
fn v_again_cancels_the_selection() {
    let mut preview = Preview::open(&write("cancel", "main.rs", CODE));
    preview.handle(PanelKey::Char('v'));
    preview.handle(PanelKey::Down);
    preview.handle(PanelKey::Char('v'));
    assert!(!preview.is_selecting());
    assert_eq!(preview.selection(), (1, 1), "back to just the cursor line");
}

#[test]
fn a_selects_the_whole_file() {
    let mut preview = Preview::open(&write("all", "main.rs", CODE));
    preview.set_height(10);
    preview.handle(PanelKey::Char('a'));
    assert_eq!(preview.selection(), (0, 4));
}

// Shift and an arrow should select without having to enter a mode first.
#[test]
fn shift_arrows_select_without_pressing_v() {
    let mut preview = Preview::open(&write("shift", "main.rs", CODE));
    preview.set_height(10);

    preview.handle(PanelKey::SelectDown);
    preview.handle(PanelKey::SelectDown);
    assert!(preview.is_selecting());
    assert_eq!(preview.selection(), (0, 2));
    assert_eq!(preview.selected_lines(), 3);
}

#[test]
fn shift_arrows_keep_extending_an_existing_selection() {
    let mut preview = Preview::open(&write("extend", "main.rs", CODE));
    preview.handle(PanelKey::Char('v'));
    preview.handle(PanelKey::SelectDown);
    assert_eq!(preview.selection(), (0, 1));
    preview.handle(PanelKey::SelectDown);
    assert_eq!(preview.selection(), (0, 2), "the anchor did not move");
}
