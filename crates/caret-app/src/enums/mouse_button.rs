#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    WheelUp,
    WheelDown,
    /// Motion with nothing held.
    None,
}
