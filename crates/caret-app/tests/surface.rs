use caret_app::types::corners::Corners;
use caret_app::types::surface::Surface;
use caret_app::types::surface_cell::SurfaceCell;
use caret_core::{Flags, Grapheme, Rgb};
use caret_layout::Rect;

const BG: Rgb = Rgb::new(0, 0, 0);
const FILL: Rgb = Rgb::new(40, 40, 40);

#[test]
fn a_rounded_fill_marks_all_four_corners() {
    let mut surface = Surface::new(20, 10, BG);
    let rect = Rect::new(2, 2, 6, 4);
    surface.fill_rounded(rect, FILL, BG);

    let corner = |x, y| surface.get(x, y).expect("cell").round;
    assert_eq!(corner(2, 2), Corners::TOP_LEFT, "top left");
    assert_eq!(corner(7, 2), Corners::TOP_RIGHT, "top right");
    assert_eq!(corner(2, 5), Corners::BOTTOM_LEFT, "bottom left");
    assert_eq!(corner(7, 5), Corners::BOTTOM_RIGHT, "bottom right");
}

#[test]
fn only_the_corners_are_marked() {
    let mut surface = Surface::new(20, 10, BG);
    surface.fill_rounded(Rect::new(2, 2, 6, 4), FILL, BG);

    assert_eq!(surface.get(4, 2).unwrap().round, Corners::NONE, "top edge");
    assert_eq!(surface.get(2, 3).unwrap().round, Corners::NONE, "left edge");
    assert_eq!(surface.get(4, 4).unwrap().round, Corners::NONE, "inside");
}

// A one-row run is a pill: both of its cells round top and bottom.
#[test]
fn a_single_row_rounds_into_a_pill() {
    let mut surface = Surface::new(20, 10, BG);
    surface.fill_rounded(Rect::new(3, 4, 5, 1), FILL, BG);

    assert_eq!(surface.get(3, 4).unwrap().round, Corners::LEFT);
    assert_eq!(surface.get(7, 4).unwrap().round, Corners::RIGHT);
}

#[test]
fn a_single_cell_rounds_on_every_side() {
    let mut surface = Surface::new(20, 10, BG);
    surface.fill_rounded(Rect::new(5, 5, 1, 1), FILL, BG);

    let round = surface.get(5, 5).unwrap().round;
    for corner in [
        Corners::TOP_LEFT,
        Corners::TOP_RIGHT,
        Corners::BOTTOM_LEFT,
        Corners::BOTTOM_RIGHT,
    ] {
        assert!(round.contains(corner), "{corner:?}");
    }
}

// A rounded cell carries what is behind it: a selected row sits on a panel,
// not on the window, and painting the window's colour outside its arc would
// punch a hole in the panel.
#[test]
fn a_rounded_cell_remembers_what_it_sits_on() {
    let mut surface = Surface::new(20, 10, BG);
    let under = Rgb::new(30, 30, 30);
    surface.fill_rounded(Rect::new(3, 4, 5, 1), FILL, under);

    assert_eq!(surface.get(3, 4).unwrap().under, under, "the rounded end");
    assert_eq!(surface.get(5, 4).unwrap().bg, FILL, "the middle is plain");
}

// A label on a pill used to square its ends off again, because writing a
// glyph reset the cell's rounding along with everything else.
#[test]
fn writing_over_a_pill_keeps_its_rounded_ends() {
    let mut surface = Surface::new(20, 10, BG);
    let rect = Rect::new(2, 2, 5, 1);
    surface.fill_rounded(rect, FILL, BG);
    surface.text_over(
        2,
        2,
        5,
        "12345",
        caret_app::types::style::Style::new(BG, FILL),
    );

    assert_eq!(surface.get(2, 2).expect("cell").ch, '1');
    assert_eq!(surface.get(2, 2).expect("cell").round, Corners::LEFT);
    assert_eq!(surface.get(6, 2).expect("cell").round, Corners::RIGHT);
    assert_eq!(surface.get(6, 2).expect("cell").under, BG);
}

#[test]
fn writing_over_nothing_leaves_the_surface_alone() {
    let mut surface = Surface::new(4, 2, BG);
    surface.text_over(
        9,
        9,
        5,
        "out of bounds",
        caret_app::types::style::Style::new(BG, FILL),
    );
    assert_eq!(surface.get(0, 0).expect("cell").ch, ' ');
}

// A frame reaches the window, or the terminal it was started from, as one
// write: whatever a cell shows has to survive that trip whole.
#[test]
fn a_cell_showing_more_than_one_character_keeps_them_all() {
    let mut surface = Surface::new(6, 1, BG);
    let handle = surface.hold("q\u{0330}");
    surface.set(
        0,
        0,
        SurfaceCell {
            ch: handle,
            fg: FILL,
            bg: BG,
            flags: Flags::NONE,
            round: Corners::NONE,
            under: BG,
        },
    );

    let cell = surface.get(0, 0).expect("the cell");
    assert_eq!(surface.grapheme(&cell), Grapheme::Many("q\u{0330}"));
    assert!(
        surface.render().contains("q\u{0330}"),
        "and is written out as both characters"
    );
}

#[test]
fn an_ordinary_cell_is_its_own_character() {
    let surface = Surface::new(4, 1, BG);
    let cell = surface.get(0, 0).expect("the cell");
    assert_eq!(surface.grapheme(&cell), Grapheme::One(' '));
}
