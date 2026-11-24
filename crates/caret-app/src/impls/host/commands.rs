#[allow(unused_imports)]
use crate::prelude::*;

use caret_config::FinishNotice;

/// A command that took longer than this is worth saying how long it took.
const WORTH_TIMING: std::time::Duration = std::time::Duration::from_secs(2);

/// What to do about a command that has finished.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Notice {
    /// Say nothing: it was quick, it worked, or nobody asked.
    Nothing,
    /// The status line, which reaches somebody looking at the window.
    Status,
    /// And a notification, which reaches somebody who is not.
    System,
}

/// Whether a finished command is worth announcing, and how far.
///
/// Three things decide it: whether anybody was watching that pane, how
/// long it ran, and how far the configuration lets the news travel. A
/// command in the pane you are looking at never announces itself, because
/// you just watched it happen.
pub fn notice_for(
    how: FinishNotice,
    after: std::time::Duration,
    took: Option<std::time::Duration>,
    watched: bool,
    window_focused: bool,
) -> Notice {
    if how == FinishNotice::Off || watched {
        return Notice::Nothing;
    }
    if !took.is_some_and(|took| took >= after) {
        return Notice::Nothing;
    }
    match how {
        FinishNotice::System if !window_focused => Notice::System,
        _ => Notice::Status,
    }
}

impl Session {
    /// The shell says a command has finished.
    pub(crate) fn command_finished(&mut self, id: PaneId, status: Option<i32>) {
        let Some(pane) = self.pane_mut(id) else {
            return;
        };
        let took = pane.running_since.take().map(|at| at.elapsed());
        pane.last_status = status;
        pane.last_duration = took;

        let watched = id == self.workspace.active_pane() && self.window_focused;
        let notice = notice_for(
            self.config.terminal.notify_finish,
            std::time::Duration::from_secs(self.config.terminal.notify_after as u64),
            took,
            watched,
            self.window_focused,
        );

        let failed = status.is_some_and(|code| code != 0);
        let slow = took.is_some_and(|took| took >= WORTH_TIMING);
        if !failed && !slow && notice == Notice::Nothing {
            return;
        }

        let outcome = match status {
            Some(code) if code != 0 => format!("exit {code}"),
            _ => "done".to_string(),
        };
        let timing = took
            .filter(|took| *took >= WORTH_TIMING)
            .map(|took| format!(" after {}", spoken(took)))
            .unwrap_or_default();

        if notice == Notice::Nothing {
            self.message = format!(" {outcome}{timing}");
        } else {
            let name = self.pane_label(id);
            self.message = format!(" {name}: {outcome}{timing}");
            if notice == Notice::System {
                notify(&format!("{name}: {outcome}{timing}"));
            }
        }
        self.needs_draw = true;
    }

    /// What to call a pane when saying something about it from elsewhere.
    fn pane_label(&self, id: PaneId) -> String {
        match self.pane_number(id) {
            Some(number) => format!("pane {number}"),
            None if Some(id) == self.agent => "the agent".to_string(),
            None => "a pane".to_string(),
        }
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

/// Raises a notification with whatever the desktop has for the purpose.
///
/// Nothing leaves this machine, and nothing is said if the platform has
/// no such thing: a missing notifier is not worth an error in a terminal.
fn notify(text: &str) {
    let mut command = if cfg!(target_os = "macos") {
        let mut osascript = std::process::Command::new("osascript");
        osascript.arg("-e").arg(format!(
            "display notification {} with title \"caret\"",
            quoted(text)
        ));
        osascript
    } else {
        let mut send = std::process::Command::new("notify-send");
        send.arg("caret").arg(text);
        send
    };
    let _ = command
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
}

/// A string as AppleScript reads it, which is the one place a pane's name
/// becomes part of a script rather than text.
fn quoted(text: &str) -> String {
    format!("\"{}\"", text.replace('\\', "\\\\").replace('"', "\\\""))
}

/// A duration as somebody would say it.
fn spoken(took: std::time::Duration) -> String {
    let seconds = took.as_secs();
    if seconds < 60 {
        return format!("{seconds}s");
    }
    format!("{}m{}s", seconds / 60, seconds % 60)
}
