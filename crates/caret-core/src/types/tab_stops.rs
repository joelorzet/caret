#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct TabStops {
    pub(crate) bits: Box<[u64]>,
    pub(crate) cols: u16,
}
