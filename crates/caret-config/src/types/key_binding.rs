#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct KeyBinding {
    pub mods: Modifiers,
    pub key: Key,
}
