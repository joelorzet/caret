#[allow(unused_imports)]
use crate::prelude::*;

/// Turns raw terminal input into key presses.
///
/// Holds a partial escape sequence between reads, since a keypress can arrive
/// split across two `read` calls.
#[derive(Default)]
pub struct InputDecoder {
    pub(crate) pending: Vec<u8>,
}
