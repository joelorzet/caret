#[allow(unused_imports)]
use crate::prelude::*;

/// What the last check found, so the next one can be skipped.
///
/// Kept beside the configuration but not in it: the configuration is the
/// user's file and nothing writes to it behind their back.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct CheckState {
    /// Seconds since the epoch, or zero for never.
    pub checked_at: u64,
    /// The newest tag the last check saw, for reporting without asking again.
    pub seen: String,
}
