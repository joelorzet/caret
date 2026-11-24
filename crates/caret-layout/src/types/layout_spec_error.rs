#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LayoutSpecError {
    pub input: String,
    pub reason: &'static str,
    pub at: usize,
}
