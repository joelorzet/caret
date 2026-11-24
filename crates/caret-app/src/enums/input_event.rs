#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub enum InputEvent {
    Key(KeyPress),
    Mouse(MouseEvent),
    /// A bracketed paste from the surrounding terminal.
    Paste(Vec<u8>),
}
