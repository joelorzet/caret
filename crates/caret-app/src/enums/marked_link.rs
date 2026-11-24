#[allow(unused_imports)]
use crate::prelude::*;

/// What caret is willing to make of a target a program marked.
///
/// A path is still only a claim at this point: whether the file is there
/// decides whether the link is offered, exactly as for a path caret found
/// by itself.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MarkedLink {
    Url(String),
    Path(String),
}
