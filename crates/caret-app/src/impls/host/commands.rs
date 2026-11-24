#[allow(unused_imports)]
use crate::prelude::*;

/// A command that took longer than this is worth saying how long it took.
const WORTH_TIMING: std::time::Duration = std::time::Duration::from_secs(2);

impl Session {
    /// The shell says a command has finished.
    pub(crate) fn command_finished(&mut self, id: PaneId, status: Option<i32>) {
        let Some(pane) = self.pane_mut(id) else {
            return;
        };
        let took = pane.running_since.take().map(|at| at.elapsed());
        pane.last_status = status;
        pane.last_duration = took;

        let failed = status.is_some_and(|code| code != 0);
        let slow = took.is_some_and(|took| took >= WORTH_TIMING);
        if !failed && !slow {
            return;
        }

        let mut said = match status {
            Some(code) if code != 0 => format!(" exit {code}"),
            _ => " done".to_string(),
        };
        if let Some(took) = took.filter(|took| *took >= WORTH_TIMING) {
            said.push_str(&format!(" after {}", spoken(took)));
        }
        self.message = said;
        self.needs_draw = true;
    }

    /// Scrolls to the prompt before or after the one on screen.
    ///
    /// Measured from the top row in view rather than from the cursor: this
    /// is for reading back through what happened, and the cursor is at the
    /// bottom where the next command will be typed.
    pub(crate) fn jump_command(&mut self, back: bool) {
        let id = self.workspace.active_pane();
        let Some(pane) = self.pane_mut(id) else {
            return;
        };
        let screen = pane.terminal.screen_mut();
        let from = screen.display_row(0);
        let found = if back {
            screen.prompt_above(from)
        } else {
            screen.prompt_below(from)
        };

        match found {
            Some(row) => {
                screen.scroll_to_row(row);
                self.message = String::new();
            }
            None if screen.prompt_count() == 0 => {
                // Without the shell's marks there is nothing to jump to,
                // and saying so beats a key that silently does nothing.
                self.message = " no command marks: the shell has not sent any".into();
            }
            None => {
                self.message = if back {
                    " no earlier command".into()
                } else {
                    " no later command".into()
                }
            }
        }
        self.needs_draw = true;
    }
}

/// A duration as somebody would say it.
fn spoken(took: std::time::Duration) -> String {
    let seconds = took.as_secs();
    if seconds < 60 {
        return format!("{seconds}s");
    }
    format!("{}m{}s", seconds / 60, seconds % 60)
}
