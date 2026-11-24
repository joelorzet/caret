#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SelectionMode {
    /// Character by character.
    Cell,
    /// Whole words, from a double click.
    Word,
    /// Whole lines, from a triple click.
    Line,
}
