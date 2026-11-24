#[allow(unused_imports)]
use crate::prelude::*;

/// How far each arm of a box character reaches: 0 none, 1 light, 2 heavy.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct BoxArms {
    pub up: u8,
    pub down: u8,
    pub left: u8,
    pub right: u8,
}
