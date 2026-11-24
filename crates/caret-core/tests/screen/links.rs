use crate::common::write;
use caret_core::Screen;

/// Marks `text` as a link to `target`, the way a program does.
fn marked(s: &mut Screen, text: &str, target: &str) {
    s.open_link(target);
    write(s, text);
    s.close_link();
}

/// The row the cursor is on, counted the way the screen counts rows.
fn current_row(s: &Screen) -> u32 {
    s.history_len() + s.cursor().y as u32
}

#[test]
fn a_marked_link_opens_a_target_that_is_not_the_text() {
    let mut s = Screen::new(40, 10, 100);
    let row = current_row(&s);
    marked(&mut s, "the failing test", "https://ci.example/run/7");

    let (start, end, target) = s.marked_link(row, 4).expect("a link under the text");
    assert_eq!(target, "https://ci.example/run/7");
    assert_eq!((start, end), (0, "the failing test".len()));
    assert_eq!(s.row_text(row), "the failing test");
}

#[test]
fn text_beside_a_link_is_not_part_of_it() {
    let mut s = Screen::new(40, 10, 100);
    let row = current_row(&s);
    write(&mut s, "see ");
    marked(&mut s, "here", "https://example.com/a");
    write(&mut s, " now");

    assert!(s.marked_link(row, 0).is_none(), "the words before it");
    assert!(s.marked_link(row, 9).is_none(), "the words after it");
    let (start, end, target) = s.marked_link(row, 5).expect("the link itself");
    assert_eq!((start, end, target), (4, 8, "https://example.com/a"));
}

#[test]
fn two_links_on_one_row_keep_their_own_targets() {
    let mut s = Screen::new(40, 10, 100);
    let row = current_row(&s);
    marked(&mut s, "one", "https://example.com/1");
    write(&mut s, " ");
    marked(&mut s, "two", "https://example.com/2");

    assert_eq!(
        s.marked_link(row, 1).map(|l| l.2),
        Some("https://example.com/1")
    );
    assert_eq!(
        s.marked_link(row, 5).map(|l| l.2),
        Some("https://example.com/2")
    );
}

// The target rides in the cells, so everything that moves cells moves it.
#[test]
fn a_link_survives_scrolling_back_to_it() {
    let mut s = Screen::new(40, 5, 100);
    let row = current_row(&s);
    marked(&mut s, "a report", "https://example.com/report");
    write(&mut s, "\r\n");
    for i in 0..20 {
        write(&mut s, &format!("line {i}\r\n"));
    }

    assert_eq!(s.row_text(row), "a report");
    assert_eq!(
        s.marked_link(row, 2).map(|l| l.2),
        Some("https://example.com/report"),
        "the row scrolled into history and kept its link"
    );
}

#[test]
fn a_link_survives_the_window_changing_width() {
    let mut s = Screen::new(20, 5, 100);
    marked(
        &mut s,
        "a link worth keeping across a rewrap",
        "https://example.com/x",
    );

    s.resize(60, 5);
    let found = (0..s.history_len() + s.rows() as u32)
        .filter_map(|row| (0..60).find_map(|col| s.marked_link(row, col)))
        .next()
        .expect("the link is still somewhere");
    assert_eq!(found.2, "https://example.com/x");
}

#[test]
fn erasing_a_link_leaves_plain_text_behind() {
    let mut s = Screen::new(40, 10, 100);
    let row = current_row(&s);
    marked(&mut s, "gone", "https://example.com/gone");
    s.carriage_return();
    write(&mut s, "kept");

    assert_eq!(s.row_text(row), "kept");
    assert!(
        s.marked_link(row, 1).is_none(),
        "writing over a link takes the target with it"
    );
}

#[test]
fn colour_changing_in_the_middle_of_a_link_does_not_end_it() {
    let mut s = Screen::new(40, 10, 100);
    let row = current_row(&s);
    s.open_link("https://example.com/coloured");
    write(&mut s, "red");
    s.reset_pen();
    write(&mut s, "plain");
    s.close_link();

    let (start, end, target) = s.marked_link(row, 6).expect("one link, not two");
    assert_eq!((start, end), (0, 8));
    assert_eq!(target, "https://example.com/coloured");
}

// A listing can mark every name it prints, and the targets have to stop
// somewhere. What they must never do is start answering with each other's.
#[test]
fn a_pane_that_marks_endless_links_forgets_the_old_ones_rather_than_confusing_them() {
    let mut s = Screen::new(40, 10, 10_000);
    for i in 0..6_000 {
        marked(&mut s, "name", &format!("https://example.com/{i}"));
        write(&mut s, "\r\n");
    }

    for row in 0..s.history_len() + s.rows() as u32 {
        if let Some((_, _, target)) = s.marked_link(row, 0) {
            let text = s.row_text(row);
            assert!(
                target.starts_with("https://example.com/"),
                "row {row} ({text:?}) answered with {target:?}"
            );
        }
    }
    let newest = s.history_len() + s.rows() as u32 - 2;
    assert!(
        s.marked_link(newest, 0).is_some(),
        "the links just printed still work"
    );
}

#[test]
fn a_pane_with_no_marked_links_is_no_bigger_for_the_feature() {
    let plain = Screen::new(200, 50, 10_000);
    let mut used = Screen::new(200, 50, 10_000);
    let before = used.memory_bytes();
    assert_eq!(before, plain.memory_bytes());

    marked(&mut used, "one link", "https://example.com/one");
    let target = "https://example.com/one".len();
    let grew = used.memory_bytes() - before;
    assert!(
        grew >= target && grew < target + 128,
        "one link cost {grew} bytes: its own target and nothing per cell"
    );
}
