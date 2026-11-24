//! Chrome geometry in pixels.
//!
//! The terminal grid is addressed in cells, which is right for text and wrong
//! for everything around it: a tab bar quantised to the cell box can only ever
//! be a row of coloured spaces. This crate lays chrome out in free pixels so
//! the painter can draw it with real proportions, and keeps the arithmetic
//! separate from the drawing so it can be tested on its own.

pub mod enums;
pub mod impls;
pub mod types;

mod prelude;

pub use crate::enums::strip_hit::StripHit;
pub use crate::types::px_rect::PxRect;
pub use crate::types::strip_layout::StripLayout;
pub use crate::types::tab_item::TabItem;
pub use crate::types::tab_strip::TabStrip;
