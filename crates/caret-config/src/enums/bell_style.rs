#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum BellStyle {
    None,
    #[default]
    Visual,
    Audible,
}
