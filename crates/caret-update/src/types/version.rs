#[allow(unused_imports)]
use crate::prelude::*;

/// A released version, as `major.minor.patch`.
///
/// Anything after the numbers, `-rc1` and the like, marks a version as
/// coming before the plain one it is named after, which is what semantic
/// versioning says and what everybody expects.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    /// Empty for a plain release.
    pub pre: String,
}
