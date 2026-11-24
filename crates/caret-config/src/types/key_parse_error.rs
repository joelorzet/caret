#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KeyParseError {
    pub input: String,
    pub reason: &'static str,
}
