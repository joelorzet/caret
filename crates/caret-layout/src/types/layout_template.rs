#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct LayoutTemplate {
    pub name: String,
    pub description: String,
    pub shape: LayoutShape,
}
