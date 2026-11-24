#[allow(unused_imports)]
use crate::prelude::*;

/// The handful of rules that make a language legible.
///
/// Not a parser: comments, strings, numbers and keywords are what the eye
/// uses to read code at a glance, and they can be found with a scan.
pub struct Syntax {
    pub line_comments: &'static [&'static str],
    pub block_comment: Option<(&'static str, &'static str)>,
    pub quotes: &'static [char],
    pub keywords: &'static [&'static str],
}
