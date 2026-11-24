#[allow(unused_imports)]
use crate::prelude::*;

/// The targets of the marked links a pane has printed, one entry per
/// [`LinkId`].
///
/// Empty until a program marks something, so a pane that never does pays
/// nothing. When it fills, every target is forgotten at once and the cells
/// pointing at them are swept back to no link, which keeps the rule that an
/// id either names its own target or names nothing: a recycled id that still
/// had cells behind it would open the wrong page, and a link that quietly
/// becomes plain text is the lesser of those.
pub struct LinkTable {
    pub(crate) targets: Vec<Box<str>>,
    pub(crate) generation: u32,
}
