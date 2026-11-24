#[allow(unused_imports)]
use crate::prelude::*;

/// Kept beside the configuration rather than in it: the configuration is
/// the user's file and nothing writes to it behind their back.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct CheckState {
    /// Seconds since the epoch, zero for never.
    pub checked_at: u64,
    pub seen: String,
}
