//! Escape sequence parsing: turns a PTY byte stream into mutations on a
//! [`caret_core::Terminal`].
//!
//! `vte` provides the state machine; this crate is the handler that maps each
//! dispatched sequence onto the terminal state. It performs no I/O, so every
//! sequence is testable by feeding bytes in and inspecting the grid.
//!
//! `types/` holds declarations, `impls/` holds the code.

pub mod enums;
pub mod impls;
pub mod types;

mod prelude;

pub use crate::enums::dec_mode::{AnsiMode, DecMode};
pub use crate::types::parser::Parser;
pub use crate::types::performer::Performer;
