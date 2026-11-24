#[allow(unused_imports)]
use crate::prelude::*;

/// What a URL starts with, as it appears in output.
const SCHEMES: [&str; 2] = ["https://", "http://"];

/// Characters that end a link however they are used: a link inside quotes
/// or brackets stops at them rather than swallowing them.
const STOPS: &str = " \t\"'`<>()[]{}|";

/// Punctuation that ends a sentence rather than a link.
const TRAILING: &str = ".,;:!?";

/// Every link on a line, in the order they appear.
pub fn links_in(line: &str) -> Vec<Link> {
    let chars: Vec<char> = line.chars().collect();
    let mut links: Vec<Link> = Vec::new();
    let mut at = 0;

    while at < chars.len() {
        if let Some(link) = url_at(&chars, at) {
            at = link.end;
            links.push(link);
            continue;
        }
        if let Some(link) = path_at(&chars, at) {
            at = link.end;
            links.push(link);
            continue;
        }
        at += 1;
    }
    links
}

/// The link under a column, if there is one.
pub fn link_at(line: &str, column: usize) -> Option<Link> {
    links_in(line)
        .into_iter()
        .find(|link| column >= link.start && column < link.end)
}

/// A URL beginning exactly at `at`.
fn url_at(chars: &[char], at: usize) -> Option<Link> {
    let rest: String = chars[at..].iter().collect();
    let scheme = SCHEMES.iter().find(|scheme| rest.starts_with(**scheme))?;

    let mut end = at + scheme.chars().count();
    while end < chars.len() && !STOPS.contains(chars[end]) {
        end += 1;
    }
    // A link at the end of a sentence does not own the full stop.
    while end > at && TRAILING.contains(chars[end - 1]) {
        end -= 1;
    }
    let text: String = chars[at..end].iter().collect();
    if text.chars().count() <= scheme.chars().count() {
        return None;
    }
    Some(Link {
        start: at,
        end,
        target: LinkTarget::Url(text),
    })
}

/// A path beginning exactly at `at`, with the line and column a compiler
/// may have put after it.
fn path_at(chars: &[char], at: usize) -> Option<Link> {
    if at > 0 && !STOPS.contains(chars[at - 1]) {
        return None;
    }
    let mut end = at;
    while end < chars.len() && !STOPS.contains(chars[end]) {
        end += 1;
    }
    while end > at && TRAILING.contains(chars[end - 1]) {
        end -= 1;
    }
    let text: String = chars[at..end].iter().collect();

    let (path, line, column) = split_place(&text);
    // Something with no separator in it is a word, not a path. A path that
    // is only punctuation is nothing at all.
    if !path.contains('/') || path.chars().all(|c| STOPS.contains(c) || c == '/') {
        return None;
    }

    Some(Link {
        start: at,
        end,
        target: LinkTarget::File {
            path: path.to_string(),
            line,
            column,
        },
    })
}

/// `src/main.rs:42:9` is a path and a place in it, which is how every
/// compiler and test runner says where to look.
fn split_place(text: &str) -> (&str, Option<u32>, Option<u16>) {
    let mut parts = text.split(':');
    let path = parts.next().unwrap_or(text);
    let line = parts.next().and_then(|n| n.parse().ok());
    let column = parts.next().and_then(|n| n.parse().ok());
    (path, line, column.filter(|_| line.is_some()))
}
