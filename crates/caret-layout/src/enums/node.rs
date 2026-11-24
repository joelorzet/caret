#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone)]
pub enum Node {
    Leaf(PaneId),
    Split(Split),
}
