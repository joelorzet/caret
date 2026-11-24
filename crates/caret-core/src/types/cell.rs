#[allow(unused_imports)]
use crate::prelude::*;

/// Exactly 16 bytes, asserted below. A 200x10,000 scrollback is then 32 MiB in
/// one allocation.
///
/// `link` sits in what used to be tail padding: the four fields before it
/// come to fourteen bytes and the alignment rounded that up to sixteen
/// anyway, so a marked link rides along for nothing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
    pub flags: Flags,
    pub link: LinkId,
}

const _: () = assert!(core::mem::size_of::<Cell>() == 16);
