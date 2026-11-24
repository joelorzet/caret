#[allow(unused_imports)]
use crate::prelude::*;

/// A box-drawing or block character, described geometrically.
///
/// Drawing these from the font leaves gaps: a `\u{2502}` glyph rarely spans
/// the full cell height, so a column of them reads as a dotted line rather
/// than a rule. Describing the shape lets the renderer draw rectangles that
/// meet exactly at the cell edges.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BoxGlyph {
    /// Arms reaching from the centre to each edge.
    Lines(BoxArms),
    /// A quarter turn joining two arms, for the rounded corners.
    Arc(BoxCorner),
    Block(BlockShape),
}

/// Fraction of the cell a block character covers.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlockShape {
    Full,
    UpperHalf,
    LowerHalf,
    LeftHalf,
    RightHalf,
    /// Eighths filled from the left, 1 through 7.
    LeftEighths(u8),
    /// Eighths filled from the bottom, 1 through 7.
    LowerEighths(u8),
    /// Partial coverage, 0-255, for the shade characters.
    Shade(u8),
}
