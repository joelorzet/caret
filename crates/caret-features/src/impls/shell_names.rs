#[allow(unused_imports)]
use crate::prelude::*;

/// The names a shell says it knows, out of what it printed.
///
/// Shells answer differently and none of them answer cleanly: one prints
/// `name=value`, another prefixes every line with `alias`, a third lists
/// functions as `declare -f name`, and a shell may print a line of its own
/// before any of it. A line earns a name only if it is shaped like one:
/// an assignment whose left side is a single word, or a line that is
/// nothing but a single word. Everything else is somebody talking.
pub fn shell_names(listing: &str) -> Vec<String> {
    let mut names = Vec::new();
    for line in listing.lines() {
        let line = line.trim();
        let line = line
            .strip_prefix("alias ")
            .or_else(|| line.strip_prefix("declare -f "))
            .or_else(|| line.strip_prefix("function "))
            .unwrap_or(line)
            .trim();

        let candidate = match line.split_once('=') {
            Some((name, _)) => name.trim_end_matches("()").trim(),
            None => line.trim_end_matches("()").trim(),
        };

        if is_name(candidate) {
            names.push(candidate.to_string());
        }
    }
    names.sort_unstable();
    names.dedup();
    names
}

/// Whether a word is something a shell could be asked to run.
///
/// One word, no spaces, and nothing that would make it an option rather
/// than a command.
fn is_name(word: &str) -> bool {
    !word.is_empty()
        && !word.starts_with('-')
        && word
            .chars()
            .all(|c| c.is_alphanumeric() || "_-.".contains(c))
}
