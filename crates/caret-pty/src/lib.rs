//! Pseudoterminals and the processes behind them.
//!
//! One pane is one pty and one child; there is no server process and no
//! per-pane thread. Every master fd is serviced by a single `poll`, so the
//! cost of waiting is one syscall no matter how many panes are open.
//!
//! # Platforms
//!
//! This is POSIX, not macOS-specific: `posix_openpt`, `TIOCSCTTY`, `termios`
//! and `poll` are the same on macOS and Linux, and both are supported by the
//! code here.
//!
//! Windows has no pty; it has ConPTY, which is a different API shape
//! (`CreatePseudoConsole` plus overlapped pipe handles rather than a master
//! fd). The public surface of this crate — [`PtyPool`], [`PtySpec`],
//! [`PtyEvent`] — is deliberately free of fd-shaped types in its signatures so
//! a ConPTY backend can sit behind it. See `docs/platforms.md`.
//!
//! `types/` and `enums/` hold declarations, `impls/` holds the code.

pub mod enums;
pub mod impls;
pub mod types;

mod prelude;

#[cfg(not(unix))]
compile_error!(
    "caret-pty currently has only a POSIX backend (macOS and Linux). \
     Windows needs a ConPTY implementation; see docs/platforms.md."
);

pub use crate::enums::pty_event::*;
pub use crate::impls::sys::terminal_size;
pub use crate::types::pty::*;
pub use crate::types::pty_id::*;
pub use crate::types::pty_pool::*;
pub use crate::types::pty_spec::*;
