#[allow(unused_imports)]
use crate::prelude::*;

use std::time::Instant;

/// The cursor's on/off clock.
///
/// Reset on every keystroke: a cursor that vanishes mid-word is the thing
/// everyone complains about, so typing always starts a fresh on-phase.
pub struct Blink {
    pub(crate) since: Instant,
    pub(crate) last: bool,
}
