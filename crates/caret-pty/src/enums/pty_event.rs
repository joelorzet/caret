#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PtyEvent {
    Output {
        id: PtyId,
        bytes: Vec<u8>,
    },
    /// The child exited; the pty has been removed from the pool.
    Exited {
        id: PtyId,
        status: i32,
    },
}
