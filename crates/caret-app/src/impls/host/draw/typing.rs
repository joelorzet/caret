//! What is being typed: the command line's colours and the suggestion behind it.
//!
//! Part of the drawing layer: one region of the window per file, so a
//! change to one of them is a change to one file.

#[allow(unused_imports)]
use super::*;

impl Session {
    /// Colours the command line as it is typed.
    ///
    /// The shell has already echoed the characters, so this recolours the
    /// cells it wrote rather than drawing anything: the shell stays in charge
    /// of its own line editing and we only say what the words are.
    pub(super) fn draw_input_highlight(&self, surface: &mut Surface, rect: Rect) {
        if !self.config.terminal.highlight_input {
            return;
        }
        let Some(input) = self.input_on_screen() else {
            return;
        };
        // A selection has its own colours, and they say something this does
        // not; leave the line alone while one is up.
        if self
            .selection
            .as_ref()
            .is_some_and(|(owner, _)| *owner == input.pane)
        {
            return;
        }
        if input.row >= rect.h {
            return;
        }

        let spans = shell_spans(&input.text);
        // Colour says "this is a command and these are its parts". Saying it
        // about a word that would not run is worse than saying nothing, so a
        // line whose first word does not resolve is left exactly as typed.
        if !self.runs(&input.text, spans.first()) {
            return;
        }

        let y = rect.y + input.row;
        for span in &spans {
            // A later stage of a pipeline is still being typed most of the
            // time, so an unrecognised one falls back to plain rather than
            // taking the colour off the whole line.
            if span.role == ShellRole::Command && !self.runs(&input.text, Some(span)) {
                continue;
            }
            let Some((fg, flags)) = self.shell_colour(span.role) else {
                continue;
            };
            for column in input.columns.get(span.start..span.end).unwrap_or(&[]) {
                let x = rect.x + column;
                if x >= rect.right() {
                    break;
                }
                let Some(mut cell) = surface.get(x, y) else {
                    continue;
                };
                cell.fg = fg;
                cell.flags = cell.flags.union(flags);
                surface.set(x, y, cell);
            }
        }
    }

    /// Whether the command a span covers names something that would run.
    fn runs(&self, line: &str, span: Option<&ShellSpan>) -> bool {
        let Some(span) = span.filter(|s| s.role == ShellRole::Command) else {
            return false;
        };
        let word: String = line
            .chars()
            .skip(span.start)
            .take(span.end - span.start)
            .collect();
        self.commands.contains(&word)
    }

    /// What a part of a command line is drawn in.
    ///
    /// `None` leaves the cell as the shell coloured it, which is what an
    /// ordinary argument should look like.
    fn shell_colour(&self, role: ShellRole) -> Option<(Rgb, Flags)> {
        let syntax = &self.syntax;
        Some(match role {
            // The verb carries the weight: it is what you check before enter.
            ShellRole::Command => (syntax.function, Flags::BOLD),
            ShellRole::Subcommand => (syntax.type_name, Flags::NONE),
            ShellRole::Flag => (syntax.keyword, Flags::NONE),
            ShellRole::Variable => (syntax.property, Flags::NONE),
            // A path and a quoted string are both data, and reading alike is
            // the point: neither is going to be run.
            ShellRole::Quoted | ShellRole::Path => (syntax.string, Flags::NONE),
            ShellRole::Number => (syntax.number, Flags::NONE),
            ShellRole::Comment => (syntax.comment, Flags::NONE),
            // Plumbing is structure rather than content, so it is weight
            // rather than colour.
            ShellRole::Operator => (syntax.plain, Flags::BOLD),
            ShellRole::Plain => return None,
        })
    }

    /// A rule along the dock's inner edge, so it reads as a panel beside the
    /// terminal rather than as text floating in it.
    /// Ghost text for the suggestion, and the list of alternatives.
    ///
    /// Drawn over the pane rather than in a panel of its own: a completion
    /// belongs on the line it completes.
    pub(super) fn draw_suggestions(&self, surface: &mut Surface, rect: Rect) {
        if !self.autocomplete.is_active() {
            return;
        }
        let Some(pane) = self.pane(self.workspace.active_pane()) else {
            return;
        };
        let screen = pane.terminal.screen();
        if screen.scroll_offset() > 0 {
            return;
        }

        let ui = &self.theme.ui;
        let cursor = screen.cursor();
        let x = rect.x + cursor.x;
        let y = rect.y + cursor.y;

        if let Some(remainder) = self.autocomplete.remainder() {
            let room = rect.right().saturating_sub(x);
            surface.text(
                x,
                y,
                room,
                remainder,
                Style::new(ui.tab_inactive_fg, self.theme.bg),
            );
        }

        // One match is just the ghost text; a list only earns its space when
        // there is a choice to make.
        let count = self.autocomplete.match_count();
        if count < 2 {
            return;
        }

        let rows = count.min(SUGGESTION_ROWS);
        let width = self
            .autocomplete
            .matches()
            .take(rows)
            .map(|entry| entry.chars().count())
            .max()
            .unwrap_or(0)
            .min(rect.w.saturating_sub(2) as usize) as u16
            + 2;

        // Below the cursor if it fits, above it otherwise.
        let below = y + 1 + rows as u16 <= rect.bottom();
        let top = if below {
            y + 1
        } else {
            y.saturating_sub(rows as u16)
        };
        let left = x.min(rect.right().saturating_sub(width));

        for (row, entry) in self.autocomplete.matches().take(rows).enumerate() {
            let line = top + row as u16;
            if line >= rect.bottom() {
                break;
            }
            let chosen = row == self.autocomplete.selected();
            let bg = if chosen { ui.selection_ui } else { ui.panel_bg };
            surface.fill_rounded(Rect::new(left, line, width, 1), bg, self.theme.bg);
            surface.text(
                left + 1,
                line,
                width.saturating_sub(1),
                &truncate(entry, width.saturating_sub(1) as usize),
                Style {
                    fg: if chosen {
                        ui.tab_active_fg
                    } else {
                        ui.chrome_fg
                    },
                    bg,
                    flags: if chosen { Flags::BOLD } else { Flags::NONE },
                },
            );
        }

        if count > rows {
            let note = format!(" +{} more    up down choose    right accept ", count - rows);
            let line = if below { top + rows as u16 } else { y + 1 };
            if line < rect.bottom() {
                surface.text(
                    left,
                    line,
                    rect.right().saturating_sub(left),
                    &note,
                    Style::new(ui.tab_inactive_fg, ui.panel_bg),
                );
            }
        }
    }
}
