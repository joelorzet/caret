#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TerminalEvent {
    TitleChanged,
    Bell,
    /// The shell reported a new working directory (OSC 7).
    CwdChanged,
    PromptMarked,
    /// A program in the pane asked for text to be put on the clipboard
    /// (OSC 52). Whether that is allowed is the host's decision.
    ClipboardWrite(String),
}
