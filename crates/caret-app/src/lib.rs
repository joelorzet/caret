//! Window shell: owns the workspace, the terminal behind each pane, and the
//! event loop connecting input, pseudoterminals and the renderer.
//!
//! The current renderer draws into the terminal this process was started from,
//! which exercises every layer below it without a windowing dependency. A
//! windowed backend replaces `impls/host/draw.rs` and nothing else.
//!
//! `types/` holds declarations, `impls/` holds the code.

pub mod enums;
pub mod impls;
pub mod types;

mod prelude;

pub use crate::impls::host::draw::paste::notes as paste_notes;
pub use crate::impls::host::draw::{
    agent_rect, arrange_canvas, arrange_rect, dock_body, join_dividers,
};
pub use crate::impls::host::encode::encode_for_child;
pub use crate::impls::host::palette::{group_rows, palette_rect};
pub use crate::impls::host::session::{assign_pane_numbers, login_name, path_with};
pub use crate::impls::run::run;
pub use crate::types::session::Session;
