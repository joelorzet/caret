#[allow(unused_imports)]
use crate::prelude::*;

/// Characters that end one command and begin another, so the next word is a
/// program name again.
const SEPARATORS: [char; 6] = ['|', '&', ';', '(', ')', '\n'];

/// Characters that are plumbing wherever they appear.
const OPERATORS: [char; 8] = ['|', '&', ';', '<', '>', '(', ')', '\n'];

/// What makes an argument look like a place rather than a word.
const PATH_MARKS: [char; 2] = ['/', '~'];

/// Splits a command line into coloured runs.
///
/// This is a highlighter, not a parser: it never has to be right about what
/// the shell will do, only about what the person reading it meant. So it
/// gives up on anything clever and keeps the common shapes exact.
pub fn shell_spans(line: &str) -> Vec<ShellSpan> {
    let chars: Vec<char> = line.chars().collect();
    let mut spans = Vec::new();
    let mut at = 0;
    // Where the next word stands: a program, that program's verb, or neither.
    let mut expect = Expect::Command;

    while at < chars.len() {
        let c = chars[at];

        if c.is_whitespace() {
            at += 1;
            continue;
        }

        if c == '#' && starts_word(&chars, at) {
            spans.push(ShellSpan {
                start: at,
                end: chars.len(),
                role: ShellRole::Comment,
            });
            break;
        }

        if c == '\'' || c == '"' {
            let end = closing_quote(&chars, at, c);
            spans.push(ShellSpan {
                start: at,
                end,
                role: ShellRole::Quoted,
            });
            at = end;
            expect = expect.after_word();
            continue;
        }

        if OPERATORS.contains(&c) {
            let end = run_of(&chars, at, |ch| OPERATORS.contains(&ch));
            spans.push(ShellSpan {
                start: at,
                end,
                role: ShellRole::Operator,
            });
            if chars[at..end].iter().any(|ch| SEPARATORS.contains(ch)) {
                expect = Expect::Command;
            }
            at = end;
            continue;
        }

        if c == '$' {
            let end = variable_end(&chars, at);
            spans.push(ShellSpan {
                start: at,
                end,
                role: ShellRole::Variable,
            });
            at = end;
            expect = expect.after_word();
            continue;
        }

        let end = word_end(&chars, at);
        let word: String = chars[at..end].iter().collect();
        spans.extend(word_spans(&word, at, expect));
        expect = if is_assignment(&word) {
            // `FOO=bar cmd` still runs cmd, so the command slot is untouched.
            expect
        } else {
            expect.after_word()
        };
        at = end;
    }

    spans
}

/// Where a word stands in the line.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Expect {
    Command,
    Subcommand,
    Argument,
}

impl Expect {
    fn after_word(self) -> Expect {
        match self {
            Expect::Command => Expect::Subcommand,
            _ => Expect::Argument,
        }
    }
}

/// Splits one word, which may itself be `NAME=value`.
fn word_spans(word: &str, start: usize, expect: Expect) -> Vec<ShellSpan> {
    let len = word.chars().count();
    let end = start + len;

    if let Some(name) = assignment_name(word) {
        let split = start + name.chars().count();
        let mut out = vec![
            ShellSpan {
                start,
                end: split,
                role: ShellRole::Variable,
            },
            ShellSpan {
                start: split,
                end: split + 1,
                role: ShellRole::Operator,
            },
        ];
        if split + 1 < end {
            out.push(ShellSpan {
                start: split + 1,
                end,
                role: role_of_argument(&word[name.len() + 1..]),
            });
        }
        return out;
    }

    let role = if word.starts_with('-') {
        // `--name=value` colours the name as the flag and the rest as what it
        // is, which is how you spot a value that went to the wrong flag.
        if let Some(eq) = word.find('=') {
            let split = start + word[..eq].chars().count();
            return vec![
                ShellSpan {
                    start,
                    end: split,
                    role: ShellRole::Flag,
                },
                ShellSpan {
                    start: split,
                    end: split + 1,
                    role: ShellRole::Operator,
                },
                ShellSpan {
                    start: split + 1,
                    end,
                    role: role_of_argument(&word[eq + 1..]),
                },
            ];
        }
        ShellRole::Flag
    } else {
        match expect {
            Expect::Command => ShellRole::Command,
            // Only a word that could be a verb takes the verb slot. `cat
            // src/main.rs` and `sleep 30` put a place and a number there,
            // and colouring those as verbs is worse than not colouring them.
            Expect::Subcommand => match role_of_argument(word) {
                ShellRole::Plain => ShellRole::Subcommand,
                other => other,
            },
            Expect::Argument => role_of_argument(word),
        }
    };

    vec![ShellSpan { start, end, role }]
}

fn role_of_argument(word: &str) -> ShellRole {
    if word.is_empty() {
        return ShellRole::Plain;
    }
    if word.chars().all(|c| c.is_ascii_digit() || c == '.')
        && word.chars().any(|c| c.is_ascii_digit())
    {
        return ShellRole::Number;
    }
    if word.chars().any(|c| PATH_MARKS.contains(&c)) || word.starts_with('.') {
        return ShellRole::Path;
    }
    ShellRole::Plain
}

/// `NAME=` at the front of a word, where NAME is a shell-legal name.
fn assignment_name(word: &str) -> Option<&str> {
    let eq = word.find('=')?;
    let name = &word[..eq];
    if name.is_empty() || name.starts_with(|c: char| c.is_ascii_digit()) {
        return None;
    }
    name.chars()
        .all(|c| c.is_alphanumeric() || c == '_')
        .then_some(name)
}

fn is_assignment(word: &str) -> bool {
    assignment_name(word).is_some()
}

fn starts_word(chars: &[char], at: usize) -> bool {
    at == 0 || chars[at - 1].is_whitespace()
}

/// Past the closing quote, or to the end of the line if it never came: a
/// half-typed string is still a string.
fn closing_quote(chars: &[char], open: usize, quote: char) -> usize {
    let mut at = open + 1;
    while at < chars.len() {
        if chars[at] == '\\' && quote == '"' {
            at += 2;
            continue;
        }
        if chars[at] == quote {
            return at + 1;
        }
        at += 1;
    }
    chars.len()
}

fn variable_end(chars: &[char], at: usize) -> usize {
    let mut end = at + 1;
    if end < chars.len() && chars[end] == '{' {
        while end < chars.len() && chars[end] != '}' {
            end += 1;
        }
        return (end + 1).min(chars.len());
    }
    while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
        end += 1;
    }
    // A lone `$` is just a dollar.
    if end == at + 1 {
        at + 1
    } else {
        end
    }
}

fn word_end(chars: &[char], at: usize) -> usize {
    let mut end = at;
    while end < chars.len() {
        let c = chars[end];
        // A word stops at a variable too, so `${A}/${B}` is two variables
        // with a separator rather than one long word.
        if c.is_whitespace() || OPERATORS.contains(&c) || c == '\'' || c == '"' || c == '$' {
            break;
        }
        end += 1;
    }
    end.max(at + 1)
}

fn run_of(chars: &[char], at: usize, mut take: impl FnMut(char) -> bool) -> usize {
    let mut end = at;
    while end < chars.len() && take(chars[end]) {
        end += 1;
    }
    end.max(at + 1)
}
