#[allow(unused_imports)]
use crate::prelude::*;

/// Puts the controlling terminal in raw mode and restores it on drop.
///
/// Restoring from `Drop` matters: a panic that skipped it would leave the
/// user's shell with no echo and no line editing.
pub struct RawMode {
    pub(crate) saved: libc::termios,
}
