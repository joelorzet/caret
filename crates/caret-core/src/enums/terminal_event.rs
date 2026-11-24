#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TerminalEvent {
    TitleChanged,
    Bell,
    /// The shell reported a new working directory (OSC 7).
    CwdChanged,
    PromptMarked,
}
