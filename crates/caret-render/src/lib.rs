//! Font discovery and cell metrics.
//!
//! This is what a renderer needs before it can draw: which fonts exist, which
//! of them are monospaced, which one the configuration actually resolves to,
//! and how large a cell that makes. The windowed renderer will live here too;
//! the host renderer in `caret-app` draws through the surrounding terminal and
//! needs none of it.
//!
//! `types/` and `enums/` hold declarations, `impls/` holds the code.

pub mod enums;
pub mod impls;
pub mod types;

mod prelude;

pub use crate::enums::box_corner::BoxCorner;
pub use crate::enums::box_glyph::*;
pub use crate::enums::font_slant::*;
pub use crate::enums::glyph_style::*;
pub use crate::types::box_arms::*;
pub use crate::types::cell_metrics::*;
pub use crate::types::font_face::*;
pub use crate::types::font_family::*;
pub use crate::types::font_library::*;
pub use crate::types::glyph::*;
pub use crate::types::glyph_cache::*;
