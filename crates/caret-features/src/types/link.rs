#[allow(unused_imports)]
use crate::prelude::*;

/// Something in the output worth opening, and where it sits on the line.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Link {
    /// Character offsets into the logical line, end exclusive.
    pub start: usize,
    pub end: usize,
    pub target: LinkTarget,
}

/// What opening it would mean.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LinkTarget {
    Url(String),
    /// A path, with the place in it a compiler named, if it named one.
    File {
        path: String,
        line: Option<u32>,
        column: Option<u16>,
    },
}
