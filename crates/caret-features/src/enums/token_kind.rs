#[allow(unused_imports)]
use crate::prelude::*;

/// What a run of characters is, so the host can colour it from the theme.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TokenKind {
    Comment,
    /// A quoted string.
    Text,
    Number,
    Keyword,
    /// An identifier starting with a capital: a type in most languages.
    Type,
    /// An identifier being called.
    Function,
    /// A key on the left of `=` or `:`.
    Property,
    Plain,
}
