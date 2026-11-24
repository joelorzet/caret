#[allow(unused_imports)]
use crate::prelude::*;

/// What a check came back with.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Outcome {
    /// Nothing newer is published.
    Current,
    /// A newer release exists.
    Newer(Release),
    /// The question could not be asked: no network, no curl, a rate limit.
    /// Not an error worth interrupting anybody over.
    Unknown,
}
