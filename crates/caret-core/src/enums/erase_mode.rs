#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EraseMode {
    ToEnd,
    ToStart,
    All,
}
