#[allow(unused_imports)]
use crate::prelude::*;

/// When a paste is shown before it is sent.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum PasteConfirm {
    /// Every paste, however small.
    Always,
    /// Only one that carries a line break, a control character, or more
    /// text than anybody reads.
    #[default]
    Notable,
    /// Never: pasting runs whatever was pasted.
    Never,
}
