#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct RowMeta(pub(crate) u8);
