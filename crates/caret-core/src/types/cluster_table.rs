#[allow(unused_imports)]
use crate::prelude::*;

/// The text of the cells that hold more than one character.
///
/// A cell is sixteen flat bytes and holds one `char`, which is what makes a
/// 200x10,000 scrollback one 32 MiB allocation rather than two million
/// strings. A letter with an accent after it, or an emoji built out of
/// several code points, does not fit that, and paying for it on every cell
/// would cost more than the grid itself.
///
/// So the rare cell points here instead: its `char` is a handle from a
/// private-use range nothing else prints, its `CLUSTER` flag says to look
/// the handle up, and the text lives in this table once. Nothing on the
/// drawing path consults it unless the flag is set.
pub struct ClusterTable {
    pub(crate) text: Vec<Box<str>>,
}
