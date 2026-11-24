#[allow(unused_imports)]
use crate::prelude::*;

/// Whether caret looks for a newer release, and how often.
///
/// The check asks GitHub where `releases/latest` points and compares the tag
/// with this build. It sends the program's name and version and nothing
/// else: no identifiers, nothing about the machine or the person at it.
/// Turn it off and caret never touches the network.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct UpdateConfig {
    pub check: bool,
    /// Hours between checks, 1 to 720.
    pub interval_hours: u32,
}
