use crate::support::{write, CODE};
use caret_features::{Language, PanelKey, PanelOutcome, Preview, TokenKind};
use std::fs;

#[test]
fn a_file_is_read_numbered_and_highlighted() {
    let mut preview = Preview::open(&write("read", "main.rs", CODE));
    preview.set_height(10);

    assert!(preview.error().is_none());
    assert_eq!(preview.len(), 5);
    assert_eq!(preview.language(), Language::Rust);

    let rows: Vec<(usize, String)> = preview
        .visible()
        .map(|(index, line, _)| (index, line.to_string()))
        .collect();
    assert_eq!(rows[0], (0, "pub fn main() {".to_string()));
    assert_eq!(rows[2].1.trim(), "// a note");

    let comment_line = preview.visible().nth(2).unwrap();
    assert!(comment_line.2.iter().any(|t| t.kind == TokenKind::Comment));
}

#[test]
fn a_missing_file_reports_why() {
    let preview = Preview::open(std::path::Path::new("/nonexistent/caret/file.rs"));
    assert!(preview.error().is_some());
    assert!(preview.is_empty());
}

// Spraying a binary through the panel would print control characters.
#[test]
fn a_binary_file_is_refused_rather_than_shown() {
    let path = write("binary", "blob.bin", "");
    fs::write(&path, [0x7f, 0x45, 0x4c, 0x46, 0x00, 0x01, 0x02]).unwrap();
    let preview = Preview::open(&path);
    assert_eq!(preview.error(), Some("binary file"));
}

#[test]
fn tabs_become_spaces_so_columns_line_up() {
    let preview = Preview::open(&write("tabs", "t.rs", "a\tb\n"));
    let (_, line, _) = preview.visible().next().unwrap();
    assert_eq!(line, "a    b");
}

#[test]
fn an_empty_file_has_nothing_to_move_through() {
    let mut preview = Preview::open(&write("empty", "e.txt", ""));
    assert!(preview.is_empty());
    assert_eq!(preview.handle(PanelKey::Down), PanelOutcome::Ignored);
    assert_eq!(preview.selected_text(), "");
}

#[test]
fn the_selected_line_count_is_reported() {
    let mut preview = Preview::open(&write("count", "main.rs", CODE));
    assert_eq!(preview.selected_lines(), 1, "just the cursor line");
    preview.handle(PanelKey::Char('a'));
    assert_eq!(preview.selected_lines(), 5);
}
