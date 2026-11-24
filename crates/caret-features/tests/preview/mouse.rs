use crate::support::write;
use caret_features::{PanelKey, Preview};

#[test]
fn a_click_moves_the_cursor_and_clears_any_selection() {
    let body: String = (0..30).map(|i| format!("line {i}\n")).collect();
    let mut preview = Preview::open(&write("click", "x.txt", &body));
    preview.set_height(10);

    preview.handle(PanelKey::Char('v'));
    preview.handle(PanelKey::Down);
    assert!(preview.is_selecting());

    preview.click(4);
    assert_eq!(preview.cursor(), 4);
    assert!(!preview.is_selecting(), "a fresh click starts over");
}

#[test]
fn dragging_selects_a_range() {
    let body: String = (0..30).map(|i| format!("line {i}\n")).collect();
    let mut preview = Preview::open(&write("drag", "x.txt", &body));
    preview.set_height(10);

    preview.click(2);
    preview.drag(6);
    assert_eq!(preview.selection(), (2, 6));
    assert_eq!(preview.selected_lines(), 5);
}

// A click is in screen rows, so it has to account for how far we scrolled.
#[test]
fn clicks_account_for_scrolling() {
    let body: String = (0..60).map(|i| format!("line {i}\n")).collect();
    let mut preview = Preview::open(&write("clickscroll", "x.txt", &body));
    preview.set_height(10);
    preview.handle(PanelKey::End);

    let offset = preview.offset();
    preview.click(0);
    assert_eq!(preview.cursor(), offset, "the top visible row");
}

#[test]
fn the_wheel_scrolls_without_moving_the_cursor() {
    let body: String = (0..60).map(|i| format!("line {i}\n")).collect();
    let mut preview = Preview::open(&write("wheel", "x.txt", &body));
    preview.set_height(10);

    preview.scroll(5);
    assert_eq!(preview.offset(), 5);
    assert_eq!(preview.cursor(), 0, "the cursor stayed put");

    preview.scroll(-100);
    assert_eq!(preview.offset(), 0, "clamped at the top");
}
