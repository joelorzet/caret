//! Whether a newer caret has been released.
//!
//! The work is split so that all of it except the network call is a pure
//! function over strings: comparing two versions, reading a tag out of a
//! redirect, deciding whether a check is due, and keeping the little file
//! that remembers when the last one was. The call itself is one process.

mod enums;
mod impls;
mod prelude;
mod types;

pub use enums::outcome::Outcome;
pub use impls::check::{check, fetch_latest, RELEASES};
pub use impls::state::state_path;
pub use types::check_state::CheckState;
pub use types::release::Release;
pub use types::version::Version;
