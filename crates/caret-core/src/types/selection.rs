#[allow(unused_imports)]
use crate::prelude::*;

pub struct Selection {
    pub(crate) anchor: Point,
    pub(crate) head: Point,
    pub(crate) mode: SelectionMode,
}
