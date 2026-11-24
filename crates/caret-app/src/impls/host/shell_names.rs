#[allow(unused_imports)]
use crate::prelude::*;

use std::sync::mpsc;

/// What to ask each shell for. Anything unrecognised is asked only for its
/// aliases, which every shell understands.
fn probe(shell: &str) -> &'static str {
    match std::path::Path::new(shell)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
    {
        "zsh" => "alias; print -rl -- ${(k)functions}",
        "bash" => "alias; declare -F",
        "fish" => "alias; functions -n",
        _ => "alias",
    }
}

impl Session {
    /// Asks the shell what names it knows, so a line that runs through an
    /// alias is coloured like anything else that runs.
    ///
    /// Off the drawing path: an interactive shell reads a profile, which can
    /// take as long as somebody's configuration takes, and the first prompt
    /// is not waiting for that.
    pub(crate) fn begin_shell_names(&mut self) {
        if self.shell_names.is_some() {
            return;
        }
        let shell = self.config.shell_program();
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let _ = tx.send(ask(&shell));
        });
        self.shell_names = Some(rx);
    }

    pub(crate) fn settle_shell_names(&mut self) {
        let Some(rx) = &self.shell_names else {
            return;
        };
        let names = match rx.try_recv() {
            Ok(names) => names,
            Err(mpsc::TryRecvError::Empty) => return,
            Err(mpsc::TryRecvError::Disconnected) => Vec::new(),
        };
        self.shell_names = None;
        if names.is_empty() {
            return;
        }
        self.commands.add(names);
        self.needs_draw = true;
    }
}

/// Runs the shell once and reads what it says. A shell that cannot answer,
/// or is not a shell at all, simply says nothing.
fn ask(shell: &str) -> Vec<String> {
    let Ok(out) = std::process::Command::new(shell)
        .arg("-i")
        .arg("-c")
        .arg(probe(shell))
        .stdin(std::process::Stdio::null())
        .output()
    else {
        return Vec::new();
    };
    caret_features::shell_names(&String::from_utf8_lossy(&out.stdout))
}
