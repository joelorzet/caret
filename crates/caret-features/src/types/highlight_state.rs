#[allow(unused_imports)]
use crate::prelude::*;

/// Carries a block comment or an unterminated string across line boundaries.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct HighlightState {
    pub(crate) in_block_comment: bool,
}
