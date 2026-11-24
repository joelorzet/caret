#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Keymap {
    pub(crate) prefix: Option<KeyBinding>,
    pub(crate) direct: Vec<(KeyBinding, Action)>,
    pub(crate) prefixed: Vec<(KeyBinding, Action)>,
}
