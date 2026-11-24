#[allow(unused_imports)]
use crate::prelude::*;

/// Builds one row. Kept local so the table below reads as a table.
fn row(section: &'static str, key: &'static str, value: String, about: &'static str) -> Setting {
    let table = match section {
        "theme" => "",
        "explorer" => "sidebar.explorer",
        "agent" => "sidebar.agent",
        other => other,
    };
    Setting {
        section,
        table,
        key,
        value,
        about,
    }
}

impl Config {
    /// Every setting the config file accepts, with its current value.
    ///
    /// In file order, so what you read here is what you go and edit.
    pub fn settings(&self) -> Vec<Setting> {
        let t = &self.terminal;
        let f = &self.font;
        let w = &self.window;
        let c = &self.chrome;
        let explorer = &self.sidebars.explorer;
        let agent = &self.sidebars.agent;

        vec![
            row("theme", "theme", self.theme.clone(), "which palette to use"),
            row("font", "family", f.family.clone(), "font family name"),
            row("font", "size", f.size.to_string(), "point size"),
            row(
                "font",
                "line_height",
                f.line_height.to_string(),
                "line spacing, as a multiple",
            ),
            row(
                "font",
                "fallback",
                format!("{:?}", f.fallback),
                "families tried for a glyph the first one lacks",
            ),
            row(
                "font",
                "ligatures",
                f.ligatures.to_string(),
                "join sequences like != into one glyph",
            ),
            row(
                "font",
                "italics",
                f.italics.to_string(),
                "use the italic face for italic text",
            ),
            row(
                "font",
                "letter_spacing",
                f.letter_spacing.to_string(),
                "extra pixels between cells, -4 to 16",
            ),
            row(
                "font",
                "builtin_box_drawing",
                f.builtin_box_drawing.to_string(),
                "draw box characters ourselves, for seamless borders",
            ),
            row(
                "font",
                "bold_is_bright",
                f.bold_is_bright.to_string(),
                "bold text also uses the bright palette",
            ),
            row("window", "cols", w.cols.to_string(), "columns on opening"),
            row("window", "rows", w.rows.to_string(), "rows on opening"),
            row(
                "window",
                "padding",
                w.padding.to_string(),
                "cells of margin around the panes",
            ),
            row(
                "window",
                "opacity",
                w.opacity.to_string(),
                "window transparency, 0 to 1",
            ),
            row(
                "window",
                "decorations",
                w.decorations.to_string(),
                "draw the system title bar",
            ),
            row(
                "terminal",
                "shell",
                t.shell.clone().unwrap_or_else(|| "$SHELL".into()),
                "program each pane runs",
            ),
            row(
                "terminal",
                "shell_args",
                format!("{:?}", t.shell_args),
                "arguments passed to it",
            ),
            row(
                "terminal",
                "scrollback",
                t.scrollback.to_string(),
                "lines kept above the screen",
            ),
            row(
                "terminal",
                "cursor",
                t.cursor.name().to_string(),
                "block, bar or underline",
            ),
            row(
                "terminal",
                "cursor_fill",
                t.cursor_fill.name().to_string(),
                "solid, blink or translucent",
            ),
            row(
                "terminal",
                "cursor_color",
                t.cursor_color
                    .map(|c| format!("#{:02x}{:02x}{:02x}", c.r, c.g, c.b))
                    .unwrap_or_else(|| "from theme".into()),
                "overrides the theme's cursor colour",
            ),
            row(
                "terminal",
                "bell",
                t.bell.name().to_string(),
                "none, visual or audible",
            ),
            row(
                "terminal",
                "word_separators",
                t.word_separators.clone(),
                "characters that bound a word",
            ),
            row(
                "terminal",
                "confirm_close",
                t.confirm_close.to_string(),
                "ask before closing a busy pane",
            ),
            row(
                "terminal",
                "highlight_input",
                t.highlight_input.to_string(),
                "colour the command line as you type",
            ),
            row(
                "terminal",
                "mouse",
                t.mouse.to_string(),
                "report mouse events to programs",
            ),
            row(
                "chrome",
                "tab_bar",
                c.tab_bar.to_string(),
                "show the tab bar",
            ),
            row(
                "chrome",
                "status",
                c.status.to_string(),
                "show the status line",
            ),
            row(
                "chrome",
                "hide_tab_bar_when_single",
                c.hide_tab_bar_when_single.to_string(),
                "hide it with only one tab",
            ),
            row(
                "chrome",
                "show_memory",
                c.show_memory.to_string(),
                "show memory use in the status line",
            ),
            row(
                "chrome",
                "padding",
                c.padding.to_string(),
                "cells between the panes and the window",
            ),
            row(
                "explorer",
                "visible",
                explorer.visible.to_string(),
                "show the file explorer",
            ),
            row(
                "explorer",
                "width",
                explorer.width.to_string(),
                "its width in cells",
            ),
            row(
                "explorer",
                "show_hidden",
                explorer.show_hidden.to_string(),
                "list dotfiles",
            ),
            row(
                "explorer",
                "follow_cwd",
                explorer.follow_cwd.to_string(),
                "follow the shell's directory",
            ),
            row(
                "agent",
                "visible",
                agent.visible.to_string(),
                "show the agent panel",
            ),
            row(
                "agent",
                "width",
                agent.width.to_string(),
                "its width in cells",
            ),
            row(
                "agent",
                "command",
                agent.command.clone(),
                "what the panel runs",
            ),
            row(
                "agent",
                "args",
                format!("{:?}", agent.args),
                "arguments passed to it",
            ),
        ]
    }
}
