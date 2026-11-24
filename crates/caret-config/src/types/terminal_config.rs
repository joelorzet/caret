#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct TerminalConfig {
    pub scrollback: u32,
    /// `None` means use `$SHELL`.
    pub shell: Option<String>,
    pub shell_args: Vec<String>,
    pub cursor: CursorShape,
    pub cursor_fill: CursorFill,
    /// Overrides the theme's cursor colour when set.
    pub cursor_color: Option<Rgb>,
    pub bell: BellStyle,
    /// Characters that bound a word for double-click selection.
    pub word_separators: String,
    pub confirm_close: bool,
    /// Colour the command line as it is typed: the program, its flags, its
    /// variables and its strings. Off leaves the shell's own colours alone.
    pub highlight_input: bool,
    /// Report mouse events. Turning this off gives the surrounding terminal
    /// its own selection back when running in host mode.
    pub mouse: bool,
}
