use crate::support::{write, CODE};
use caret_features::{PanelKey, PanelOutcome, Preview};

#[test]
fn the_cursor_moves_and_stops_at_both_ends() {
    let mut preview = Preview::open(&write("move", "main.rs", CODE));
    preview.set_height(3);

    preview.handle(PanelKey::Down);
    assert_eq!(preview.cursor(), 1);

    for _ in 0..20 {
        preview.handle(PanelKey::Down);
    }
    assert_eq!(preview.cursor(), 4, "stops at the last line");

    preview.handle(PanelKey::Home);
    assert_eq!(preview.cursor(), 0);
    preview.handle(PanelKey::End);
    assert_eq!(preview.cursor(), 4);
}

#[test]
fn the_view_scrolls_to_keep_the_cursor_visible() {
    let body: String = (0..50).map(|i| format!("line {i}\n")).collect();
    let mut preview = Preview::open(&write("scroll", "x.txt", &body));
    preview.set_height(5);

    preview.handle(PanelKey::End);
    let visible: Vec<usize> = preview.visible().map(|(index, _, _)| index).collect();
    assert_eq!(visible.len(), 5);
    assert!(visible.contains(&49), "the last line is on screen");

    preview.handle(PanelKey::Home);
    assert_eq!(preview.offset(), 0);
}

#[test]
fn paging_moves_about_a_screen() {
    let body: String = (0..50).map(|i| format!("line {i}\n")).collect();
    let mut preview = Preview::open(&write("page", "x.txt", &body));
    preview.set_height(10);

    preview.handle(PanelKey::PageDown);
    assert_eq!(preview.cursor(), 9);
    preview.handle(PanelKey::PageUp);
    assert_eq!(preview.cursor(), 0);
}

#[test]
fn vim_keys_move_like_the_arrows() {
    let body: String = (0..30).map(|i| format!("line {i}\n")).collect();
    let mut preview = Preview::open(&write("vim", "x.txt", &body));
    preview.set_height(10);

    preview.handle(PanelKey::Char('j'));
    preview.handle(PanelKey::Char('j'));
    assert_eq!(preview.cursor(), 2);
    preview.handle(PanelKey::Char('k'));
    assert_eq!(preview.cursor(), 1);

    preview.handle(PanelKey::Char('G'));
    assert_eq!(preview.cursor(), 29, "G goes to the last line");
    preview.handle(PanelKey::Char('g'));
    assert_eq!(preview.cursor(), 0, "g goes to the first");
}

#[test]
fn half_page_keys_move_half_a_screen() {
    let body: String = (0..60).map(|i| format!("line {i}\n")).collect();
    let mut preview = Preview::open(&write("half", "x.txt", &body));
    preview.set_height(20);

    preview.handle(PanelKey::HalfPageDown);
    assert_eq!(preview.cursor(), 10);
    preview.handle(PanelKey::HalfPageUp);
    assert_eq!(preview.cursor(), 0);
}

#[test]
fn escape_and_q_and_left_all_close_it() {
    for key in [PanelKey::Escape, PanelKey::Char('q'), PanelKey::Left] {
        let mut preview = Preview::open(&write("close", "main.rs", CODE));
        assert_eq!(preview.handle(key), PanelOutcome::Dismiss, "{key:?}");
    }
}

#[test]
fn e_asks_for_the_file_to_be_opened_in_an_editor() {
    let path = write("editor", "main.rs", CODE);
    let mut preview = Preview::open(&path);
    assert_eq!(
        preview.handle(PanelKey::Char('e')),
        PanelOutcome::OpenEditor(path)
    );
}
