#[allow(unused_imports)]
use crate::prelude::*;

/// Tag in the high byte:
///   `0x00______` default   `0x010000ii` palette slot   `0x02rrggbb` direct
///
/// Packed rather than an enum so a cell stays 16 bytes.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct Color(pub(crate) u32);
