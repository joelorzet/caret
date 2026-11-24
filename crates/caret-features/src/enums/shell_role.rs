#[allow(unused_imports)]
use crate::prelude::*;

/// What a run of a shell command line is, so the host can colour it.
///
/// Shell-shaped rather than language-shaped: a command line has a verb, the
/// things it is being told to do, and the plumbing between them, and those
/// are the distinctions worth seeing while typing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShellRole {
    /// The program being run: the first word, or the first after a pipe.
    Command,
    /// The word after the command, where a tool takes a verb of its own:
    /// `git commit`, `cargo build`.
    Subcommand,
    /// `-v`, `--long`, and the `=` form's name.
    Flag,
    /// `$NAME`, `${NAME}`, and the name of an assignment.
    Variable,
    /// Anything between quotes, the quotes included.
    Quoted,
    /// Pipes, redirections, separators and the brackets around them.
    Operator,
    /// An argument that looks like a place: has a slash, or starts at home.
    Path,
    Number,
    /// `#` to the end of the line.
    Comment,
    /// An ordinary argument.
    Plain,
}
