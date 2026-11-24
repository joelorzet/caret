//! Whether a newer caret has been released.
//!
//! Everything but the request itself is a pure function over strings.

mod enums;
mod impls;
mod prelude;
mod types;

pub use enums::outcome::Outcome;
pub use impls::check::{check, download, fetch_latest, RELEASES};
pub use impls::install::{hash_of, install, pick_asset};
pub use impls::state::{now, state_path};
pub use types::check_state::CheckState;
pub use types::release::Release;
pub use types::version::Version;
