#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Outcome {
    Current,
    Newer(Release),
    /// The question could not be asked, which is not an error worth
    /// interrupting anybody over.
    Unknown,
}
