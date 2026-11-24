#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TerminalEvent {
    TitleChanged,
    Bell,
    /// The shell reported a new working directory (OSC 7).
    CwdChanged,
    PromptMarked,
    /// The shell says a command's output begins here.
    CommandStarted,
    /// The shell says the command finished, with its exit status where it
    /// gave one.
    CommandFinished(Option<i32>),
    /// A program in the pane asked for text to be put on the clipboard
    /// (OSC 52). Whether that is allowed is the host's decision.
    ClipboardWrite(String),
}
