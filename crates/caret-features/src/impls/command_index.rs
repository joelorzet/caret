#[allow(unused_imports)]
use crate::prelude::*;

use std::collections::HashSet;

/// Words a shell runs without looking at `PATH`: builtins, and the keywords
/// that start a compound command.
const SHELL_WORDS: &[&str] = &[
    "alias", "bg", "bind", "break", "builtin", "case", "cd", "command", "continue", "declare",
    "do", "done", "echo", "elif", "else", "esac", "eval", "exec", "exit", "export", "false", "fc",
    "fg", "fi", "for", "function", "getopts", "hash", "help", "history", "if", "in", "jobs",
    "kill", "let", "local", "logout", "popd", "printf", "pushd", "pwd", "read", "readonly",
    "return", "select", "set", "shift", "shopt", "source", "test", "then", "time", "times", "trap",
    "true", "type", "typeset", "ulimit", "umask", "unalias", "unset", "until", "wait", "where",
    "which", "while",
];

impl CommandIndex {
    /// Reads `PATH` once and remembers what it found.
    pub fn from_path() -> CommandIndex {
        let mut names: HashSet<String> = SHELL_WORDS.iter().map(|w| w.to_string()).collect();

        let Some(path) = std::env::var_os("PATH") else {
            return CommandIndex { names };
        };
        for dir in std::env::split_paths(&path) {
            let Ok(entries) = std::fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries.flatten() {
                // Not checking the mode: a name on `PATH` that is not
                // executable is rare, and a stat per entry is not.
                if let Some(name) = entry.file_name().to_str() {
                    names.insert(name.to_string());
                }
            }
        }
        CommandIndex { names }
    }

    /// Adds names the shell knows and `PATH` does not: its aliases and its
    /// functions, which run exactly like anything else.
    pub fn add(&mut self, names: impl IntoIterator<Item = String>) {
        self.names.extend(names);
    }

    /// Whether `word` names something that would run.
    ///
    /// A word with a slash in it is a path the person wrote out, so it is
    /// taken at its word rather than stat-ed on every keystroke. Aliases and
    /// functions arrive through `add` once the shell has been asked; until
    /// then they read as unrecognised, which errs towards saying nothing.
    pub fn contains(&self, word: &str) -> bool {
        if word.is_empty() {
            return false;
        }
        if word.contains('/') {
            return true;
        }
        self.names.contains(word)
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
}
