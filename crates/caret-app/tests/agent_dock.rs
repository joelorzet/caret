use caret_app::dock_body;
use caret_layout::Rect;

/// A dock as the layout hands it over: the card, plus the one-column gap that
/// separates it from the panes and carries the focus rule.
const RIGHT: Rect = Rect {
    x: 80,
    y: 2,
    w: 40,
    h: 30,
};
const LEFT: Rect = Rect {
    x: 0,
    y: 2,
    w: 30,
    h: 30,
};

// The rule is drawn in the gap, on the dock's own first column. Anything that
// starts there, as the cursor did, lands on top of it.
#[test]
fn the_right_docks_contents_start_past_the_focus_rule() {
    let body = dock_body(RIGHT, false);
    assert!(body.x > RIGHT.x, "{} vs {}", body.x, RIGHT.x);
}

#[test]
fn the_left_docks_contents_stop_short_of_the_focus_rule() {
    let body = dock_body(LEFT, true);
    assert!(body.right() < LEFT.right());
}

#[test]
fn both_docks_hold_their_contents_off_the_card_by_the_same_amount() {
    let right = dock_body(RIGHT, false);
    let left = dock_body(LEFT, true);
    assert_eq!(RIGHT.w - right.w, LEFT.w - left.w);
    assert_eq!(RIGHT.h - right.h, LEFT.h - left.h);
}

#[test]
fn a_docks_contents_stay_inside_the_space_it_was_given() {
    for (dock, on_left) in [(RIGHT, false), (LEFT, true)] {
        let body = dock_body(dock, on_left);
        assert!(body.x >= dock.x);
        assert!(body.y >= dock.y);
        assert!(body.right() <= dock.right());
        assert!(body.bottom() <= dock.bottom());
    }
}

// A dock squeezed down to nothing must collapse, not wrap around to a huge
// width and have the agent sized to it.
#[test]
fn a_dock_with_no_room_collapses() {
    for w in 0..6u16 {
        let dock = Rect::new(10, 0, w, 3);
        let body = dock_body(dock, false);
        assert!(body.w <= w, "width {w} gave {}", body.w);
        assert!(body.right() <= dock.right().max(dock.x));
    }
}

#[test]
fn the_vertical_padding_is_shallower_than_the_horizontal_one() {
    let body = dock_body(RIGHT, false);
    let horizontal = RIGHT.w - body.w;
    let vertical = RIGHT.h - body.h;
    assert!(
        horizontal > vertical,
        "a cell is taller than it is wide, so equal counts read uneven"
    );
}

// The agent is a terminal, so it gets the card itself. Padding it leaves a
// ring of panel colour, and since a terminal's background is the window's
// own, that ring reads as a grey border round the panel.
#[test]
fn the_agent_fills_its_card_rather_than_sitting_inside_it() {
    let card = caret_app::agent_rect(RIGHT);
    let padded = dock_body(RIGHT, false);
    assert!(card.w > padded.w);
    assert!(card.h > padded.h);
    assert_eq!(card.h, RIGHT.h, "nothing is taken off the top or bottom");
}

#[test]
fn the_agents_card_still_clears_the_focus_rule() {
    let card = caret_app::agent_rect(RIGHT);
    assert!(card.x > RIGHT.x);
    assert_eq!(card.right(), RIGHT.right());
}

#[test]
fn the_agents_card_collapses_with_the_dock() {
    for w in 0..3u16 {
        let card = caret_app::agent_rect(Rect::new(10, 0, w, 3));
        assert!(card.w <= w);
        assert!(card.right() <= (10 + w).max(10));
    }
}
