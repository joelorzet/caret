#[allow(unused_imports)]
use crate::prelude::*;

/// A paste waiting to be looked at.
pub struct PendingPaste {
    pub target: PaneId,
    pub text: String,
    pub review: PasteReview,
}
