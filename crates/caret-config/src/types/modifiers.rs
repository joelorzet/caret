#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct Modifiers(pub(crate) u8);
