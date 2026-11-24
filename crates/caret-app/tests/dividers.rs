use caret_app::join_dividers;
use caret_app::types::surface::Surface;
use caret_core::Rgb;
use caret_layout::{Axis, Divider, Rect};

const BG: Rgb = Rgb::new(0, 0, 0);
const LINE: Rgb = Rgb::new(90, 90, 90);

/// A surface with the dividers drawn as straight runs, as the panes leave
/// them, then joined.
fn joined(dividers: &[Divider]) -> Surface {
    let mut surface = Surface::new(20, 12, BG);
    for divider in dividers {
        let ch = match divider.axis {
            Axis::Horizontal => '\u{2502}',
            Axis::Vertical => '\u{2500}',
        };
        surface.hline(divider.rect, ch, LINE, BG);
    }
    join_dividers(&mut surface, dividers);
    surface
}

fn ch(surface: &Surface, x: u16, y: u16) -> char {
    surface.get(x, y).expect("cell").ch
}

// A column running into a row used to stop a cell short, and the gap read as
// the panes not quite lining up.
#[test]
fn a_column_meeting_a_row_from_above_becomes_a_tee() {
    let dividers = [
        Divider {
            rect: Rect::new(0, 6, 20, 1),
            axis: Axis::Vertical,
        },
        Divider {
            rect: Rect::new(8, 0, 1, 6),
            axis: Axis::Horizontal,
        },
    ];
    let surface = joined(&dividers);
    assert_eq!(ch(&surface, 8, 6), '\u{2534}', "arms up, left and right");
    assert_eq!(ch(&surface, 7, 6), '\u{2500}', "the row is untouched");
    assert_eq!(ch(&surface, 8, 5), '\u{2502}', "so is the column");
}

#[test]
fn a_column_meeting_a_row_from_below_becomes_the_other_tee() {
    let dividers = [
        Divider {
            rect: Rect::new(0, 6, 20, 1),
            axis: Axis::Vertical,
        },
        Divider {
            rect: Rect::new(8, 7, 1, 5),
            axis: Axis::Horizontal,
        },
    ];
    assert_eq!(ch(&joined(&dividers), 8, 6), '\u{252c}');
}

#[test]
fn a_column_crossing_a_row_becomes_a_cross() {
    let dividers = [
        Divider {
            rect: Rect::new(0, 6, 20, 1),
            axis: Axis::Vertical,
        },
        Divider {
            rect: Rect::new(8, 0, 1, 6),
            axis: Axis::Horizontal,
        },
        Divider {
            rect: Rect::new(8, 7, 1, 5),
            axis: Axis::Horizontal,
        },
    ];
    assert_eq!(ch(&joined(&dividers), 8, 6), '\u{253c}');
}

#[test]
fn a_row_ending_against_a_column_becomes_a_side_tee() {
    let dividers = [
        Divider {
            rect: Rect::new(8, 0, 1, 12),
            axis: Axis::Horizontal,
        },
        Divider {
            rect: Rect::new(9, 6, 11, 1),
            axis: Axis::Vertical,
        },
    ];
    assert_eq!(
        ch(&joined(&dividers), 8, 6),
        '\u{251c}',
        "arms up, down, right"
    );
}

#[test]
fn a_lone_divider_is_left_as_the_straight_run_it_is() {
    let dividers = [Divider {
        rect: Rect::new(0, 6, 20, 1),
        axis: Axis::Vertical,
    }];
    let surface = joined(&dividers);
    for x in 0..20 {
        assert_eq!(ch(&surface, x, 6), '\u{2500}', "column {x}");
    }
}

// Joining reads the cells around each divider cell, so it must not reach
// outside the surface at an edge.
#[test]
fn joining_at_the_edge_of_the_surface_is_safe() {
    let dividers = [
        Divider {
            rect: Rect::new(0, 0, 20, 1),
            axis: Axis::Vertical,
        },
        Divider {
            rect: Rect::new(0, 0, 1, 12),
            axis: Axis::Horizontal,
        },
    ];
    let surface = joined(&dividers);
    assert_eq!(
        ch(&surface, 0, 0),
        '\u{250c}',
        "only down and right: a corner"
    );
}
