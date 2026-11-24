#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct AgentConfig {
    pub visible: bool,
    pub width: u16,
    /// The agent panel is a pane like any other; this is what it runs.
    pub command: String,
    pub args: Vec<String>,
}
