#[allow(unused_imports)]
use crate::prelude::*;

/// Exactly 16 bytes, asserted below. A 200x10,000 scrollback is then 32 MiB in
/// one allocation.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
    pub flags: Flags,
}

const _: () = assert!(core::mem::size_of::<Cell>() == 16);
