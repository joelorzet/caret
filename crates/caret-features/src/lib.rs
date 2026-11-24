//! Built-in panels.
//!
//! A panel is state plus key handling; it returns lines for the host to draw
//! rather than drawing itself, which keeps this crate free of any dependency
//! on the renderer, the theme or the keybinding vocabulary.
//!
//! The agent dock is not here: it is an ordinary terminal pane that runs a
//! command, so it needs no code of its own.
//!
//! `types/` and `enums/` hold declarations, `impls/` holds the code.

pub mod enums;
pub mod impls;
pub mod types;

mod prelude;

pub use crate::enums::entry_kind::*;
pub use crate::enums::language::*;
pub use crate::enums::line_kind::*;
pub use crate::enums::panel_key::*;
pub use crate::enums::panel_outcome::*;
pub use crate::enums::shell_role::*;
pub use crate::enums::token_kind::*;
pub use crate::impls::history;
pub use crate::impls::shell_line::shell_spans;
pub use crate::types::autocomplete::*;
pub use crate::types::command_index::*;
pub use crate::types::entry::*;
pub use crate::types::explorer::*;
pub use crate::types::highlight_state::*;
pub use crate::types::panel_line::*;
pub use crate::types::preview::*;
pub use crate::types::shell_span::*;
pub use crate::types::syntax::*;
pub use crate::types::token::*;
