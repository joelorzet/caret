//! Theme model and the built-in themes.
//!
//! A theme hand-specifies the sixteen ANSI colors, background, text, cursor,
//! selection and an accent; the window chrome is derived from those so themes
//! stay short and internally consistent.
//!
//! `types/` and `enums/` hold declarations, `impls/` holds the code, and
//! `impls/builtin/` holds one file per theme.

pub mod enums;
pub mod impls;
pub mod types;

mod prelude;

pub use crate::enums::appearance::*;
pub use crate::enums::color_slot::*;
pub use crate::impls::builtin;
pub use crate::types::palette::*;
pub use crate::types::syntax_palette::*;
pub use crate::types::theme::*;
pub use crate::types::theme_spec::*;
pub use crate::types::ui_colors::*;
