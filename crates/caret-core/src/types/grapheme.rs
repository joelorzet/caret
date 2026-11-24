#[allow(unused_imports)]
use crate::prelude::*;

/// What a cell shows: one character, or a character and whatever combines
/// with it.
///
/// Most cells are `One`, which is why the cluster is kept beside the grid
/// rather than in it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Grapheme<'a> {
    One(char),
    Many(&'a str),
}
