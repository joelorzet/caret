#[allow(unused_imports)]
use crate::prelude::*;

/// Identifies a pseudoterminal within a [`PtyPool`].
///
/// The caller chooses the values, so a pane id can be used directly.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct PtyId(pub u32);
