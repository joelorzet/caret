//! Actions, keybindings and configuration.
//!
//! An invalid config never prevents startup: problems are collected as
//! warnings and the affected settings keep their defaults.
//!
//! `types/` and `enums/` hold declarations, `impls/` holds the code.

pub mod enums;
pub mod impls;
pub mod types;

mod prelude;

pub use crate::enums::action::*;
pub use crate::enums::action_group::*;
pub use crate::enums::bell_style::*;
pub use crate::enums::cursor_fill::CursorFill;
pub use crate::enums::key::*;
pub use crate::enums::key_resolution::*;
pub use crate::enums::paste_confirm::*;
pub use crate::enums::platform::*;
pub use crate::impls::config::example::EXAMPLE;
pub use crate::impls::hex::parse_hex;
pub use crate::impls::paths;
pub use crate::types::agent_config::*;
pub use crate::types::chrome_config::*;
pub use crate::types::config::*;
pub use crate::types::explorer_config::*;
pub use crate::types::font_config::*;
pub use crate::types::key_binding::*;
pub use crate::types::key_parse_error::*;
pub use crate::types::keymap::*;
pub use crate::types::loaded_config::*;
pub use crate::types::modifiers::*;
pub use crate::types::setting::*;
pub use crate::types::shortcut::*;
pub use crate::types::sidebars_config::*;
pub use crate::types::terminal_config::*;
pub use crate::types::theme_registry::*;
pub use crate::types::update_config::*;
pub use crate::types::window_config::*;
