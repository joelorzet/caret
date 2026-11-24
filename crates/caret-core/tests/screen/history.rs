use crate::common::{display, viewport, write};
use caret_core::Screen;

#[test]
fn scrolling_back_shows_history_and_clamps() {
    let mut s = Screen::new(8, 2, 10);
    for i in 0..6 {
        write(&mut s, &format!("L{i}\r\n"));
    }
    assert_eq!(s.history_len(), 5);

    assert!(s.scroll_display(3));
    assert_eq!(s.scroll_offset(), 3);
    assert_eq!(display(&s), vec!["L2", "L3"]);

    assert!(s.scroll_display(100), "clamps at the oldest row");
    assert_eq!(s.scroll_offset(), 5);
    assert!(!s.scroll_display(10), "already clamped, so no movement");

    assert!(s.scroll_to_bottom());
    assert_eq!(s.scroll_offset(), 0);
    assert_eq!(display(&s), vec!["L5", ""]);
}

// New output must not drag a scrolled-back view along with it.
#[test]
fn a_scrolled_back_view_stays_on_its_content() {
    let mut s = Screen::new(8, 2, 20);
    for i in 0..6 {
        write(&mut s, &format!("L{i}\r\n"));
    }
    s.scroll_display(3);
    let anchored = display(&s);

    write(&mut s, "new\r\n");
    assert_eq!(display(&s), anchored);
}

#[test]
fn clearing_history_keeps_the_viewport() {
    let mut s = Screen::new(8, 2, 10);
    for i in 0..6 {
        write(&mut s, &format!("L{i}\r\n"));
    }
    let before = viewport(&s);
    s.clear_history();
    assert_eq!(s.history_len(), 0);
    assert_eq!(s.scroll_offset(), 0);
    assert_eq!(viewport(&s), before);
}
