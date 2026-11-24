//! Terminal domain model: the grid, its cells, and the state an escape-sequence
//! parser mutates. No dependencies and no I/O.
//!
//! `types/` and `enums/` hold declarations, `impls/` holds the code.

pub mod enums;
pub mod impls;
pub mod types;

mod prelude;

pub use crate::enums::color_kind::*;
pub use crate::enums::cursor_shape::*;
pub use crate::enums::erase_mode::*;
pub use crate::enums::selection_mode::*;
pub use crate::enums::terminal_event::*;
pub use crate::impls::cluster_table::{cluster_handle, cluster_index};
pub use crate::impls::color::xterm_256;
pub use crate::types::cell::*;
pub use crate::types::cluster_table::*;
pub use crate::types::color::*;
pub use crate::types::cursor::*;
pub use crate::types::cursor_style::*;
pub use crate::types::flags::*;
pub use crate::types::grapheme::*;
pub use crate::types::link_id::*;
pub use crate::types::link_table::*;
pub use crate::types::modes::*;
pub use crate::types::point::*;
pub use crate::types::rgb::*;
pub use crate::types::row_meta::*;
pub use crate::types::screen::*;
pub use crate::types::scroll_region::*;
pub use crate::types::selection::*;
pub use crate::types::tab_stops::*;
pub use crate::types::terminal::*;
