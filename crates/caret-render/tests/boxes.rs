use caret_render::{BlockShape, BoxGlyph};

fn arms(ch: char) -> caret_render::BoxArms {
    match BoxGlyph::of(ch) {
        Some(BoxGlyph::Lines(arms)) => arms,
        other => panic!("{ch:?} is {other:?}, expected lines"),
    }
}

fn block(ch: char) -> BlockShape {
    match BoxGlyph::of(ch) {
        Some(BoxGlyph::Block(shape)) => shape,
        other => panic!("{ch:?} is {other:?}, expected a block"),
    }
}

#[test]
fn straight_lines_reach_both_edges() {
    let vertical = arms('│');
    assert_eq!((vertical.up, vertical.down), (1, 1));
    assert_eq!((vertical.left, vertical.right), (0, 0));

    let horizontal = arms('─');
    assert_eq!((horizontal.left, horizontal.right), (1, 1));
    assert_eq!((horizontal.up, horizontal.down), (0, 0));
}

#[test]
fn heavy_lines_are_marked_heavy() {
    assert_eq!(arms('┃').up, 2);
    assert_eq!(arms('━').left, 2);
    assert_eq!(arms('╋').weight(), 2);
    assert_eq!(arms('┼').weight(), 1);
}

#[test]
fn corners_reach_two_edges() {
    // ┌ opens down and right.
    let tl = arms('┌');
    assert_eq!((tl.down, tl.right), (1, 1));
    assert_eq!((tl.up, tl.left), (0, 0));

    let br = arms('┘');
    assert_eq!((br.up, br.left), (1, 1));
    assert_eq!((br.down, br.right), (0, 0));
}

// The rounded corners the panel border uses have to be recognised too, or the
// panel is drawn from the font and comes out broken.
#[test]
fn rounded_corners_are_arcs_rather_than_right_angles() {
    use caret_render::BoxCorner;

    for (ch, corner) in [
        ('╭', BoxCorner::TopLeft),
        ('╮', BoxCorner::TopRight),
        ('╯', BoxCorner::BottomRight),
        ('╰', BoxCorner::BottomLeft),
    ] {
        assert_eq!(BoxGlyph::of(ch), Some(BoxGlyph::Arc(corner)), "{ch:?}");
    }
}

#[test]
fn square_corners_stay_square() {
    for ch in ['┌', '┐', '└', '┘'] {
        assert!(
            matches!(BoxGlyph::of(ch), Some(BoxGlyph::Lines(_))),
            "{ch:?}"
        );
    }
}

#[test]
fn tees_and_crosses_have_the_right_arms() {
    assert_eq!(
        arms('├'),
        caret_render::BoxArms {
            up: 1,
            down: 1,
            left: 0,
            right: 1
        }
    );
    assert_eq!(
        arms('┤'),
        caret_render::BoxArms {
            up: 1,
            down: 1,
            left: 1,
            right: 0
        }
    );
    assert_eq!(
        arms('┬'),
        caret_render::BoxArms {
            up: 0,
            down: 1,
            left: 1,
            right: 1
        }
    );
    assert_eq!(
        arms('┴'),
        caret_render::BoxArms {
            up: 1,
            down: 0,
            left: 1,
            right: 1
        }
    );
    assert_eq!(
        arms('┼'),
        caret_render::BoxArms {
            up: 1,
            down: 1,
            left: 1,
            right: 1
        }
    );
}

#[test]
fn blocks_and_shades_are_recognised() {
    assert_eq!(block('█'), BlockShape::Full);
    assert_eq!(block('▀'), BlockShape::UpperHalf);
    assert_eq!(block('▄'), BlockShape::LowerHalf);
    assert_eq!(block('▌'), BlockShape::LeftHalf);
    assert_eq!(block('▐'), BlockShape::RightHalf);
    assert_eq!(block('░'), BlockShape::Shade(64));
    assert_eq!(block('▓'), BlockShape::Shade(192));
}

#[test]
fn eighth_blocks_count_the_right_way() {
    // U+2589 is seven eighths wide, U+258F is one.
    assert_eq!(block('▉'), BlockShape::LeftEighths(7));
    assert_eq!(block('▏'), BlockShape::LeftEighths(1));
    // U+2581 is one eighth tall, U+2587 is seven.
    assert_eq!(block('▁'), BlockShape::LowerEighths(1));
    assert_eq!(block('▇'), BlockShape::LowerEighths(7));
}

#[test]
fn ordinary_characters_are_left_to_the_font() {
    for ch in ['a', 'Z', '0', ' ', '|', '-', '+', '漢', '\u{0}'] {
        assert!(
            BoxGlyph::of(ch).is_none(),
            "{ch:?} should come from the font"
        );
        assert!(!BoxGlyph::is_box(ch));
    }
    assert!(BoxGlyph::is_box('│'));
}

// The junction table is the box-drawing table read backwards, so the two
// cannot disagree. These check the reading is the right way round.
#[test]
fn a_junction_is_named_by_the_arms_it_reaches_with() {
    use caret_render::BoxGlyph;
    for (up, down, left, right, expected) in [
        (true, true, true, true, '┼'),
        (true, true, false, true, '├'),
        (true, true, true, false, '┤'),
        (false, true, true, true, '┬'),
        (true, false, true, true, '┴'),
        (false, true, false, true, '┌'),
        (false, true, true, false, '┐'),
        (true, false, false, true, '└'),
        (true, false, true, false, '┘'),
        (false, false, true, true, '─'),
        (true, true, false, false, '│'),
    ] {
        assert_eq!(
            BoxGlyph::joining(up, down, left, right),
            Some(expected),
            "{up} {down} {left} {right}"
        );
    }
}

#[test]
fn nothing_joins_nothing() {
    use caret_render::BoxGlyph;
    assert_eq!(BoxGlyph::joining(false, false, false, false), None);
}

// Every junction it hands back has to read back as the arms asked for, or
// the two tables have drifted.
#[test]
fn every_junction_round_trips_through_the_table() {
    use caret_render::{BoxArms, BoxGlyph};
    for bits in 0..16u8 {
        let wanted = [bits & 1 != 0, bits & 2 != 0, bits & 4 != 0, bits & 8 != 0];
        let [up, down, left, right] = wanted;
        let Some(ch) = BoxGlyph::joining(up, down, left, right) else {
            continue;
        };
        let Some(BoxGlyph::Lines(BoxArms {
            up: u,
            down: d,
            left: l,
            right: r,
        })) = BoxGlyph::of(ch)
        else {
            panic!("{ch} is not a line glyph");
        };
        assert_eq!([u != 0, d != 0, l != 0, r != 0], wanted, "{ch}");
    }
}
