#[allow(unused_imports)]
use crate::prelude::*;

/// The names that run something: everything executable on `PATH`, plus the
/// words the shell handles itself.
///
/// Built once and kept, because `PATH` does not change under a session and
/// walking it per keystroke would be absurd.
#[derive(Clone, Default, Debug)]
pub struct CommandIndex {
    pub(crate) names: std::collections::HashSet<String>,
}
