#[allow(unused_imports)]
use crate::prelude::*;

impl BoxArms {
    const fn new(up: u8, down: u8, left: u8, right: u8) -> BoxArms {
        BoxArms {
            up,
            down,
            left,
            right,
        }
    }

    pub const fn is_empty(self) -> bool {
        self.up == 0 && self.down == 0 && self.left == 0 && self.right == 0
    }

    /// The heaviest arm, which sets the stroke width.
    pub const fn weight(self) -> u8 {
        let mut heaviest = self.up;
        if self.down > heaviest {
            heaviest = self.down;
        }
        if self.left > heaviest {
            heaviest = self.left;
        }
        if self.right > heaviest {
            heaviest = self.right;
        }
        heaviest
    }
}

/// How far an arm reaches, which is the vocabulary `BoxArms` is written in.
/// Brought into scope unqualified: the table below is read across, and a
/// prefix repeated four times a line buries the shape it is describing.
use reach::{HEAVY, LIGHT, NONE};

mod reach {
    pub const NONE: u8 = 0;
    pub const LIGHT: u8 = 1;
    pub const HEAVY: u8 = 2;
}

/// How many steps an eighth block divides a cell into.
const EIGHTHS: u8 = 7;

/// The characters this draws by hand, under the names Unicode gives them.
///
/// Written as names rather than as the characters themselves because light
/// and heavy differ by a stroke nobody can see in a source file, and as
/// names rather than as escapes because `\u{253c}` says nothing about what
/// it is. The shade and eighth blocks are ranges, so they stay as bounds.
mod glyph {
    pub const LIGHT_HORIZONTAL: char = '\u{2500}';
    pub const HEAVY_HORIZONTAL: char = '\u{2501}';
    pub const LIGHT_VERTICAL: char = '\u{2502}';
    pub const HEAVY_VERTICAL: char = '\u{2503}';

    pub const LIGHT_DOWN_RIGHT: char = '\u{250c}';
    pub const LIGHT_DOWN_LEFT: char = '\u{2510}';
    pub const LIGHT_UP_LEFT: char = '\u{2518}';
    pub const LIGHT_UP_RIGHT: char = '\u{2514}';
    pub const HEAVY_DOWN_RIGHT: char = '\u{250f}';
    pub const HEAVY_DOWN_LEFT: char = '\u{2513}';
    pub const HEAVY_UP_LEFT: char = '\u{251b}';
    pub const HEAVY_UP_RIGHT: char = '\u{2517}';

    pub const ARC_DOWN_RIGHT: char = '\u{256d}';
    pub const ARC_DOWN_LEFT: char = '\u{256e}';
    pub const ARC_UP_LEFT: char = '\u{256f}';
    pub const ARC_UP_RIGHT: char = '\u{2570}';

    pub const LIGHT_VERTICAL_RIGHT: char = '\u{251c}';
    pub const LIGHT_VERTICAL_LEFT: char = '\u{2524}';
    pub const LIGHT_DOWN_HORIZONTAL: char = '\u{252c}';
    pub const LIGHT_UP_HORIZONTAL: char = '\u{2534}';
    pub const LIGHT_CROSS: char = '\u{253c}';
    pub const HEAVY_VERTICAL_RIGHT: char = '\u{2523}';
    pub const HEAVY_VERTICAL_LEFT: char = '\u{252b}';
    pub const HEAVY_DOWN_HORIZONTAL: char = '\u{2533}';
    pub const HEAVY_UP_HORIZONTAL: char = '\u{253b}';
    pub const HEAVY_CROSS: char = '\u{254b}';

    pub const FULL_BLOCK: char = '\u{2588}';
    pub const UPPER_HALF: char = '\u{2580}';
    pub const LOWER_HALF: char = '\u{2584}';
    pub const LEFT_HALF: char = '\u{258c}';
    pub const RIGHT_HALF: char = '\u{2590}';
    pub const LIGHT_SHADE: char = '\u{2591}';
    pub const MEDIUM_SHADE: char = '\u{2592}';
    pub const DARK_SHADE: char = '\u{2593}';

    /// Filling from the left, each one step narrower than the last.
    pub const LEFT_EIGHTHS: std::ops::RangeInclusive<char> = '\u{2589}'..='\u{258f}';
    /// Filling from the bottom, each one step taller than the last.
    pub const LOWER_EIGHTHS: std::ops::RangeInclusive<char> = '\u{2581}'..='\u{2587}';
}

impl BoxGlyph {
    /// The light box character that reaches exactly these ways.
    ///
    /// The inverse of `of`, and worked out by asking `of`: a second table
    /// written out by hand would be a second thing to keep in step, and the
    /// first one is right here.
    pub fn joining(up: bool, down: bool, left: bool, right: bool) -> Option<char> {
        const JUNCTIONS: [char; 11] = [
            glyph::LIGHT_HORIZONTAL,
            glyph::LIGHT_VERTICAL,
            glyph::LIGHT_DOWN_RIGHT,
            glyph::LIGHT_DOWN_LEFT,
            glyph::LIGHT_UP_RIGHT,
            glyph::LIGHT_UP_LEFT,
            glyph::LIGHT_VERTICAL_RIGHT,
            glyph::LIGHT_VERTICAL_LEFT,
            glyph::LIGHT_DOWN_HORIZONTAL,
            glyph::LIGHT_UP_HORIZONTAL,
            glyph::LIGHT_CROSS,
        ];
        let wanted = BoxGlyph::Lines(BoxArms::new(
            u8::from(up),
            u8::from(down),
            u8::from(left),
            u8::from(right),
        ));
        JUNCTIONS
            .into_iter()
            .find(|ch| BoxGlyph::of(*ch) == Some(wanted))
    }

    /// Recognises the box-drawing and block characters worth drawing by hand.
    pub fn of(ch: char) -> Option<BoxGlyph> {
        use BlockShape::*;

        let arms =
            |up, down, left, right| Some(BoxGlyph::Lines(BoxArms::new(up, down, left, right)));
        let block = |shape| Some(BoxGlyph::Block(shape));

        match ch {
            // Lines
            glyph::LIGHT_HORIZONTAL => arms(NONE, NONE, LIGHT, LIGHT),
            glyph::HEAVY_HORIZONTAL => arms(NONE, NONE, HEAVY, HEAVY),
            glyph::LIGHT_VERTICAL => arms(LIGHT, LIGHT, NONE, NONE),
            glyph::HEAVY_VERTICAL => arms(HEAVY, HEAVY, NONE, NONE),

            // Square corners
            glyph::LIGHT_DOWN_RIGHT => arms(NONE, LIGHT, NONE, LIGHT),
            glyph::LIGHT_DOWN_LEFT => arms(NONE, LIGHT, LIGHT, NONE),
            glyph::LIGHT_UP_LEFT => arms(LIGHT, NONE, LIGHT, NONE),
            glyph::LIGHT_UP_RIGHT => arms(LIGHT, NONE, NONE, LIGHT),

            // Rounded corners, which are a quarter ellipse rather than a
            // right angle. The font's own glyphs are too small to meet their
            // neighbours, so these are worth drawing for the shape alone.
            glyph::ARC_DOWN_RIGHT => Some(BoxGlyph::Arc(BoxCorner::TopLeft)),
            glyph::ARC_DOWN_LEFT => Some(BoxGlyph::Arc(BoxCorner::TopRight)),
            glyph::ARC_UP_LEFT => Some(BoxGlyph::Arc(BoxCorner::BottomRight)),
            glyph::ARC_UP_RIGHT => Some(BoxGlyph::Arc(BoxCorner::BottomLeft)),
            glyph::HEAVY_DOWN_RIGHT => arms(NONE, HEAVY, NONE, HEAVY),
            glyph::HEAVY_DOWN_LEFT => arms(NONE, HEAVY, HEAVY, NONE),
            glyph::HEAVY_UP_LEFT => arms(HEAVY, NONE, HEAVY, NONE),
            glyph::HEAVY_UP_RIGHT => arms(HEAVY, NONE, NONE, HEAVY),

            // Tees and crosses
            glyph::LIGHT_VERTICAL_RIGHT => arms(LIGHT, LIGHT, NONE, LIGHT),
            glyph::LIGHT_VERTICAL_LEFT => arms(LIGHT, LIGHT, LIGHT, NONE),
            glyph::LIGHT_DOWN_HORIZONTAL => arms(NONE, LIGHT, LIGHT, LIGHT),
            glyph::LIGHT_UP_HORIZONTAL => arms(LIGHT, NONE, LIGHT, LIGHT),
            glyph::LIGHT_CROSS => arms(LIGHT, LIGHT, LIGHT, LIGHT),
            glyph::HEAVY_VERTICAL_RIGHT => arms(HEAVY, HEAVY, NONE, HEAVY),
            glyph::HEAVY_VERTICAL_LEFT => arms(HEAVY, HEAVY, HEAVY, NONE),
            glyph::HEAVY_DOWN_HORIZONTAL => arms(NONE, HEAVY, HEAVY, HEAVY),
            glyph::HEAVY_UP_HORIZONTAL => arms(HEAVY, NONE, HEAVY, HEAVY),
            glyph::HEAVY_CROSS => arms(HEAVY, HEAVY, HEAVY, HEAVY),

            // Blocks
            glyph::FULL_BLOCK => block(Full),
            glyph::UPPER_HALF => block(UpperHalf),
            glyph::LOWER_HALF => block(LowerHalf),
            glyph::LEFT_HALF => block(LeftHalf),
            glyph::RIGHT_HALF => block(RightHalf),
            glyph::LIGHT_SHADE => block(Shade(64)),
            glyph::MEDIUM_SHADE => block(Shade(128)),
            glyph::DARK_SHADE => block(Shade(192)),

            // Eighths: U+2589..U+258F fill from the left, U+2581..U+2587 from
            // the bottom, each one step narrower or shorter than the last.
            ch if glyph::LEFT_EIGHTHS.contains(&ch) => {
                let step = ch as u32 - *glyph::LEFT_EIGHTHS.start() as u32;
                block(LeftEighths(EIGHTHS - step as u8))
            }
            ch if glyph::LOWER_EIGHTHS.contains(&ch) => {
                let step = ch as u32 - *glyph::LOWER_EIGHTHS.start() as u32;
                block(LowerEighths(step as u8 + 1))
            }

            _ => None,
        }
    }

    /// Whether drawing this by hand is worthwhile at all.
    pub fn is_box(ch: char) -> bool {
        BoxGlyph::of(ch).is_some()
    }
}
