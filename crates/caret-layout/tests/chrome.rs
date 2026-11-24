use caret_layout::{Chrome, Rect};

const WINDOW: Rect = Rect::new(0, 0, 100, 30);

/// Dock geometry is easier to state without padding in the way; padding has
/// tests of its own below.
fn chrome() -> Chrome {
    Chrome {
        padding: 0,
        ..Chrome::default()
    }
}

#[test]
fn docks_are_hidden_by_default() {
    let f = chrome().layout(WINDOW);
    assert!(f.left.is_none());
    assert!(f.right.is_none());
    assert_eq!(f.tab_bar, Some(Rect::new(0, 0, 100, 1)));
    assert_eq!(f.status, Some(Rect::new(0, 28, 100, 2)));
    assert_eq!(f.center, Rect::new(0, 1, 100, 27));
}

#[test]
fn the_explorer_takes_space_from_the_left() {
    let mut c = chrome();
    c.left.visible = true;
    let f = c.layout(WINDOW);
    assert_eq!(f.left, Some(Rect::new(0, 1, 28, 27)));
    assert_eq!(f.center, Rect::new(28, 1, 72, 27));
}

#[test]
fn the_agent_takes_space_from_the_right() {
    let mut c = chrome();
    c.right.visible = true;
    let f = c.layout(WINDOW);
    assert_eq!(f.right, Some(Rect::new(56, 1, 44, 27)));
    assert_eq!(f.center, Rect::new(0, 1, 56, 27));
}

#[test]
fn both_docks_leave_the_panes_in_the_middle() {
    let mut c = chrome();
    c.left.visible = true;
    c.right.visible = true;
    let f = c.layout(WINDOW);

    let left = f.left.unwrap();
    let right = f.right.unwrap();
    assert_eq!(left.right(), f.center.x);
    assert_eq!(f.center.right(), right.x);
    assert_eq!(left.w + f.center.w + right.w, WINDOW.w);
}

// A narrow window must keep a usable pane area rather than squeeze the
// terminal down to nothing.
#[test]
fn a_dock_is_dropped_when_it_would_leave_no_room() {
    let mut c = chrome();
    c.left.visible = true;
    c.right.visible = true;

    let f = c.layout(Rect::new(0, 0, 60, 30));
    assert!(f.left.is_some());
    assert!(f.right.is_none(), "no room for both");
    assert!(f.center.w >= Chrome::MIN_CENTER);

    let f = c.layout(Rect::new(0, 0, 40, 30));
    assert!(f.left.is_none());
    assert_eq!(f.center.w, 40);
}

#[test]
fn chrome_is_dropped_before_the_pane_area_vanishes() {
    let f = chrome().layout(Rect::new(0, 0, 80, 1));
    assert!(f.tab_bar.is_none());
    assert!(f.status.is_none());
    assert_eq!(f.center, Rect::new(0, 0, 80, 1));
}

#[test]
fn hiding_the_tab_bar_and_status_gives_the_whole_window_to_panes() {
    let c = Chrome {
        tab_bar: 0,
        status: 0,
        ..chrome()
    };
    let f = c.layout(WINDOW);
    assert_eq!(f.center, WINDOW);
}

#[test]
fn dock_resize_is_clamped() {
    let mut c = chrome();
    c.left.resize(2);
    assert_eq!(c.left.cells, caret_layout::Dock::MIN_CELLS);
    c.left.resize(10_000);
    assert_eq!(c.left.cells, caret_layout::Dock::MAX_CELLS);
}

#[test]
fn dock_toggle_flips_visibility() {
    let mut c = chrome();
    assert!(c.left.toggle());
    assert!(c.left.visible);
    assert!(!c.left.toggle());
    assert!(!c.left.visible);
}

// --- padding ---------------------------------------------------------------

#[test]
fn padding_insets_the_pane_area_only() {
    let c = Chrome {
        padding: 2,
        ..chrome()
    };
    let f = c.layout(WINDOW);

    assert_eq!(f.center, Rect::new(2, 1, 96, 27));
    assert_eq!(
        f.tab_bar,
        Some(Rect::new(0, 0, 100, 1)),
        "the bars span the window"
    );
    assert_eq!(f.status, Some(Rect::new(0, 28, 100, 2)));
}

#[test]
fn padding_sits_inside_the_docks() {
    let mut c = Chrome {
        padding: 2,
        ..chrome()
    };
    c.left.visible = true;
    let f = c.layout(WINDOW);

    let left = f.left.unwrap();
    assert_eq!(left, Rect::new(0, 1, 28, 27), "the dock is not inset");
    assert_eq!(f.center.x, left.right() + 2);
}

// A narrow window should keep its columns for the terminal rather than spend
// them on margins.
#[test]
fn padding_is_dropped_before_the_pane_area_gets_tight() {
    let c = Chrome {
        padding: 4,
        tab_bar: 0,
        status: 0,
        ..chrome()
    };
    let narrow = Rect::new(0, 0, Chrome::MIN_CENTER + 4, 10);
    assert_eq!(c.layout(narrow).center, narrow, "no room to spare");

    let roomy = Rect::new(0, 0, 100, 10);
    assert_eq!(c.layout(roomy).center, Rect::new(4, 0, 92, 10));
}

#[test]
fn zero_padding_uses_every_column() {
    let c = Chrome {
        padding: 0,
        tab_bar: 0,
        status: 0,
        ..chrome()
    };
    assert_eq!(c.layout(WINDOW).center, WINDOW);
}
