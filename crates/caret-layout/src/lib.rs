//! Pane layout: a tmux-shaped tree of splits, tabs, saved layout templates,
//! and the window chrome around them. Pure geometry — no rendering, no
//! processes, no dependencies.
//!
//! `types/` and `enums/` hold declarations, `impls/` holds the code.

pub mod enums;
pub mod impls;
pub mod types;

mod prelude;

pub use crate::enums::axis::*;
pub use crate::enums::close_outcome::*;
pub use crate::enums::direction::*;
pub use crate::enums::dock_kind::*;
pub use crate::enums::layout_kind::*;
pub use crate::enums::layout_preset::*;
pub use crate::enums::layout_shape::*;
pub use crate::enums::node::*;
pub use crate::impls::split::distribute;
pub use crate::types::chrome::*;
pub use crate::types::divider::*;
pub use crate::types::dock::*;
pub use crate::types::frame::*;
pub use crate::types::layout_change::*;
pub use crate::types::layout_node::*;
pub use crate::types::layout_spec_error::*;
pub use crate::types::layout_template::*;
pub use crate::types::pane_id::*;
pub use crate::types::rect::*;
pub use crate::types::resolved::*;
pub use crate::types::split::*;
pub use crate::types::tab::*;
pub use crate::types::tab_id::*;
pub use crate::types::workspace::*;
