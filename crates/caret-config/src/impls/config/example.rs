#[allow(unused_imports)]
use crate::prelude::*;

/// A complete, commented config file. Written by `caret config init` and printed
/// by `caret config example`. Values shown are the defaults.
pub const EXAMPLE: &str = r##"# caret configuration
#
# Every setting is optional; anything omitted keeps its default, and the values
# below are the defaults. `caret config` shows what is actually in effect,
# `caret keys` lists the shortcuts, `caret themes` previews the themes.
#
# A value that is not understood is reported and ignored: the setting keeps
# its default and the rest of the file is still read. Nothing here can stop
# the terminal starting. `caret config` lists whatever was not understood.

# Theme name. Drop a .toml file into themes/ next to this file to add your
# own; a user theme overrides a built-in of the same name.
# `caret themes` lists every name.
theme = "tokyo-night"

[font]
family = "JetBrains Mono"   # any installed family; `caret fonts` lists them
fallback = ["SF Mono", "Menlo", "DejaVu Sans Mono", "Apple Color Emoji"]
size = 13.0                # points, 6.0 - 72.0
line_height = 1.2          # multiple of the font's own line height, 0.8 - 3.0
# Draw bold text in the bright palette slot as well as in a bold face.
bold_is_bright = false
ligatures = true           # programming ligatures, if the family has them
italics = true             # use the italic face for italic text
letter_spacing = 0.0       # extra pixels between cells, -4.0 to 16.0
builtin_box_drawing = true # draw box characters ourselves, for seamless borders

[window]
opacity = 1.0          # 0.1 - 1.0
padding = 8            # pixels around the terminal area
decorations = true
cols = 100             # initial size in cells, 20 or more
rows = 30              # 5 or more

[terminal]
scrollback = 10000     # lines kept per pane, 0 - 1000000
# shell = "/bin/zsh"   # defaults to $SHELL
shell_args = []        # a list of strings, passed as written
cursor = "block"       # block | underline | bar | hollow
# A blinking cursor blinks while the window has the keyboard and somebody has
# used it in the last few seconds, then holds steady: a blink is a whole frame
# handed to the compositor, and one cell is not worth that all day.
cursor_fill = "blink"  # blink | solid | translucent
# cursor_color = "#ff8800"   # defaults to the theme's cursor colour
bell = "visual"        # none | visual | audible
# Characters that end a word for double-click selection; any string.
word_separators = " \t\n\"'`()[]{}<>|;:,"
confirm_close = true   # ask before closing a pane that still has a job running
highlight_input = true # colour the command line as you type it
# Let a program running in a pane put text on the clipboard, which is how
# an editor's yank reaches it. Reading the clipboard is never allowed.
clipboard_write = true
# Pasting runs what you pasted, so a paste carrying a line break, a control
# character or a great deal of text is shown first.
confirm_paste = "notable"  # always | notable | never
mouse = true           # click to focus, wheel to scroll, and report to apps

[chrome]
tab_bar = true
status = true
hide_tab_bar_when_single = false
show_memory = false    # grid memory in the status line; diagnostics
padding = 1            # columns either side of the pane area, 0 to 8

# Looking for a newer release. The check asks GitHub where releases/latest
# points and compares the tag with this build: it sends caret's name and
# version and nothing else, no identifiers and nothing about you or your
# machine. With check = false caret never touches the network.
# `caret update` asks on demand, and `caret update --install` installs.
[update]
check = true
interval_hours = 24    # 1 to 720

# The file explorer, on the left.
[sidebar.explorer]
visible = false
width = 28             # 12 - 120 cells
show_hidden = false
follow_cwd = true      # track the focused pane's working directory

# The agent panel, on the right. It is an ordinary pane that runs a command,
# so it costs one child process and nothing bespoke.
[sidebar.agent]
visible = false
width = 44             # 12 - 120 cells
command = "claude"     # any program on PATH, or a path to one
args = []

[keys]
# The prefix arms the [keys.prefixed] table, tmux-style. Use "none" to turn
# prefixed bindings off entirely.
prefix = "ctrl+b"
# false starts from an empty keymap instead of the built-in one.
defaults = true
# Drop a default binding.
unbind = []

# Bindings that work on their own.
[keys.direct]
# "cmd+d" = "split_horizontal"
# "cmd+k" = "none"                 # "none" removes a binding

# Bindings that need the prefix pressed first.
[keys.prefixed]
# "%" = "split_horizontal"
# "|" = "split_horizontal"         # your own alias alongside the default

# Actions that take an amount are written action:n
# "cmd+ctrl+left" = "resize_left:10"
# "cmd+5"         = "select_tab:5"
#
# Write a shifted key as the shifted character: "cmd+D", not "cmd+shift+d".

# Saved layouts, recalled with `apply_layout:<name>`.
#
# The five presets always exist: even-horizontal, even-vertical,
# main-vertical, main-horizontal and tiled. ctrl+b alt+1..5 apply them and
# ctrl+b space cycles. A layout here with the same name replaces a preset.
#
# Either name a preset:
#
#   [layouts.dev]
#   description = "Editor left, two shells stacked right"
#   preset = "main-vertical"
#   panes = 3          # omit to rearrange however many are open
#   main = 60          # percent for the main pane
#
# Or spell the shape out, where `*` is a pane and a split is h(..) or v(..).
# A child may take a percentage; the rest share what is left:
#
#   [layouts.review]
#   description = "Wide left pane, three narrow ones right"
#   split = "h(60:*, 40:v(*, *, *))"
#
# Bind it with, in [keys.prefixed]:
#
#   "D" = "apply_layout:dev"
"##;

impl Config {
    /// Writes the example config to `path`, creating parent directories.
    /// Refuses to overwrite an existing file.
    pub fn write_example(path: &std::path::Path) -> std::io::Result<()> {
        if path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                format!("{} already exists", path.display()),
            ));
        }
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, EXAMPLE)
    }
}
