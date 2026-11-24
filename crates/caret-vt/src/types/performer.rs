#[allow(unused_imports)]
use crate::prelude::*;

/// Borrows the terminal for the duration of one `advance` call.
///
/// Keeping the borrow in a separate type is what lets `Parser` stay free of a
/// lifetime parameter while `vte` requires `&mut` access to the handler.
pub struct Performer<'a> {
    pub(crate) term: &'a mut Terminal,
}
