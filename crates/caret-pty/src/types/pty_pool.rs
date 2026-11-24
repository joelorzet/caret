#[allow(unused_imports)]
use crate::prelude::*;

/// Every pane's pseudoterminal, serviced by a single `poll` call.
///
/// A thread per pane would cost a stack and a context switch each; one `poll`
/// over every master fd costs one syscall per wake-up regardless of how many
/// panes are open.
#[derive(Default)]
pub struct PtyPool {
    pub(crate) ptys: Vec<(PtyId, Pty)>,
    /// Reused between polls so reading never allocates.
    pub(crate) scratch: Vec<u8>,
}
