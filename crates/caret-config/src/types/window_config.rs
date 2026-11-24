#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct WindowConfig {
    pub opacity: f32,
    pub padding: u16,
    pub decorations: bool,
    pub cols: u16,
    pub rows: u16,
}
