#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Release {
    pub version: Version,
    /// The tag as published, `v0.1.1`, which names the download directory.
    pub tag: String,
}
