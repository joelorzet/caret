#[allow(unused_imports)]
use crate::prelude::*;

const RUST: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while",
];
const PYTHON: &[&str] = &[
    "and", "as", "assert", "async", "await", "break", "class", "continue", "def", "del", "elif",
    "else", "except", "False", "finally", "for", "from", "global", "if", "import", "in", "is",
    "lambda", "None", "nonlocal", "not", "or", "pass", "raise", "return", "True", "try", "while",
    "with", "yield",
];
const JAVASCRIPT: &[&str] = &[
    "async",
    "await",
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "default",
    "delete",
    "do",
    "else",
    "export",
    "extends",
    "false",
    "finally",
    "for",
    "function",
    "if",
    "import",
    "in",
    "instanceof",
    "let",
    "new",
    "null",
    "return",
    "super",
    "switch",
    "this",
    "throw",
    "true",
    "try",
    "typeof",
    "var",
    "void",
    "while",
    "yield",
];
const GO: &[&str] = &[
    "break",
    "case",
    "chan",
    "const",
    "continue",
    "default",
    "defer",
    "else",
    "fallthrough",
    "for",
    "func",
    "go",
    "goto",
    "if",
    "import",
    "interface",
    "map",
    "package",
    "range",
    "return",
    "select",
    "struct",
    "switch",
    "type",
    "var",
];
const C: &[&str] = &[
    "auto", "break", "case", "char", "const", "continue", "default", "do", "double", "else",
    "enum", "extern", "float", "for", "goto", "if", "int", "long", "register", "return", "short",
    "signed", "sizeof", "static", "struct", "switch", "typedef", "union", "unsigned", "void",
    "volatile", "while",
];
const SHELL: &[&str] = &[
    "case", "do", "done", "elif", "else", "esac", "export", "fi", "for", "function", "if", "in",
    "local", "return", "then", "until", "while",
];
const JSON_LIKE: &[&str] = &["true", "false", "null"];

impl Language {
    /// Guesses from the file name, falling back to plain text.
    pub fn of(path: &std::path::Path) -> Language {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_lowercase())
            .unwrap_or_default();

        // Some files are known by name rather than by extension.
        match name.as_str() {
            "dockerfile" | "makefile" => return Language::Shell,
            "cargo.lock" => return Language::Toml,
            _ => {}
        }

        let extension = path
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_default();

        match extension.as_str() {
            "rs" => Language::Rust,
            "py" | "pyi" => Language::Python,
            "js" | "mjs" | "cjs" | "ts" | "tsx" | "jsx" => Language::JavaScript,
            "go" => Language::Go,
            "c" | "h" | "cpp" | "hpp" | "cc" | "java" | "cs" => Language::C,
            "sh" | "bash" | "zsh" | "fish" | "profile" | "bashrc" | "zshrc" => Language::Shell,
            "toml" => Language::Toml,
            "json" => Language::Json,
            "yaml" | "yml" => Language::Yaml,
            "md" | "markdown" => Language::Markdown,
            _ => Language::Plain,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Language::Rust => "rust",
            Language::Python => "python",
            Language::JavaScript => "javascript",
            Language::Go => "go",
            Language::C => "c",
            Language::Shell => "shell",
            Language::Toml => "toml",
            Language::Json => "json",
            Language::Yaml => "yaml",
            Language::Markdown => "markdown",
            Language::Plain => "text",
        }
    }

    pub const fn syntax(self) -> Syntax {
        match self {
            Language::Rust => Syntax {
                line_comments: &["//"],
                block_comment: Some(("/*", "*/")),
                quotes: &['"'],
                keywords: RUST,
            },
            Language::Python => Syntax {
                line_comments: &["#"],
                block_comment: None,
                quotes: &['"', '\''],
                keywords: PYTHON,
            },
            Language::JavaScript => Syntax {
                line_comments: &["//"],
                block_comment: Some(("/*", "*/")),
                quotes: &['"', '\'', '`'],
                keywords: JAVASCRIPT,
            },
            Language::Go => Syntax {
                line_comments: &["//"],
                block_comment: Some(("/*", "*/")),
                quotes: &['"', '`'],
                keywords: GO,
            },
            Language::C => Syntax {
                line_comments: &["//"],
                block_comment: Some(("/*", "*/")),
                quotes: &['"', '\''],
                keywords: C,
            },
            Language::Shell => Syntax {
                line_comments: &["#"],
                block_comment: None,
                quotes: &['"', '\''],
                keywords: SHELL,
            },
            Language::Toml => Syntax {
                line_comments: &["#"],
                block_comment: None,
                quotes: &['"', '\''],
                keywords: JSON_LIKE,
            },
            Language::Yaml => Syntax {
                line_comments: &["#"],
                block_comment: None,
                quotes: &['"', '\''],
                keywords: JSON_LIKE,
            },
            Language::Json => Syntax {
                line_comments: &[],
                block_comment: None,
                quotes: &['"'],
                keywords: JSON_LIKE,
            },
            Language::Markdown | Language::Plain => Syntax {
                line_comments: &[],
                block_comment: None,
                quotes: &[],
                keywords: &[],
            },
        }
    }

    /// Tokenises one line, carrying block-comment state across lines.
    ///
    /// Only the interesting runs are returned; anything not covered is plain
    /// text, which saves allocating a token per character.
    pub fn highlight(self, line: &str, state: &mut HighlightState) -> Vec<Token> {
        let syntax = self.syntax();
        // A language with no rules is plain text; highlighting stray numbers
        // in prose would be noise.
        if syntax.keywords.is_empty()
            && syntax.quotes.is_empty()
            && syntax.line_comments.is_empty()
            && syntax.block_comment.is_none()
        {
            return Vec::new();
        }

        let chars: Vec<char> = line.chars().collect();
        let mut tokens = Vec::new();
        let mut at = 0;

        if state.in_block_comment {
            let (_, close) = syntax.block_comment.unwrap_or(("", "*/"));
            match find(&chars, close, 0) {
                Some(end) => {
                    let stop = end + close.chars().count();
                    tokens.push(Token {
                        start: 0,
                        end: stop,
                        kind: TokenKind::Comment,
                    });
                    state.in_block_comment = false;
                    at = stop;
                }
                None => {
                    tokens.push(Token {
                        start: 0,
                        end: chars.len(),
                        kind: TokenKind::Comment,
                    });
                    return tokens;
                }
            }
        }

        while at < chars.len() {
            if let Some(marker) = syntax
                .line_comments
                .iter()
                .find(|marker| starts_with(&chars, marker, at))
            {
                let _ = marker;
                tokens.push(Token {
                    start: at,
                    end: chars.len(),
                    kind: TokenKind::Comment,
                });
                break;
            }

            if let Some((open, close)) = syntax.block_comment {
                if starts_with(&chars, open, at) {
                    match find(&chars, close, at + open.chars().count()) {
                        Some(end) => {
                            let stop = end + close.chars().count();
                            tokens.push(Token {
                                start: at,
                                end: stop,
                                kind: TokenKind::Comment,
                            });
                            at = stop;
                        }
                        None => {
                            tokens.push(Token {
                                start: at,
                                end: chars.len(),
                                kind: TokenKind::Comment,
                            });
                            state.in_block_comment = true;
                            break;
                        }
                    }
                    continue;
                }
            }

            let ch = chars[at];
            if syntax.quotes.contains(&ch) {
                let end = string_end(&chars, at, ch);
                tokens.push(Token {
                    start: at,
                    end,
                    kind: TokenKind::Text,
                });
                at = end;
                continue;
            }

            if ch.is_ascii_digit() {
                let mut end = at;
                while end < chars.len()
                    && (chars[end].is_ascii_alphanumeric()
                        || chars[end] == '.'
                        || chars[end] == '_')
                {
                    end += 1;
                }
                tokens.push(Token {
                    start: at,
                    end,
                    kind: TokenKind::Number,
                });
                at = end;
                continue;
            }

            if ch.is_alphabetic() || ch == '_' {
                let mut end = at;
                while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
                    end += 1;
                }
                let word: String = chars[at..end].iter().collect();
                if let Some(kind) = classify(&chars, at, end, &word, &syntax) {
                    tokens.push(Token {
                        start: at,
                        end,
                        kind,
                    });
                }
                at = end;
                continue;
            }

            at += 1;
        }

        tokens
    }
}

/// What an identifier is, from the shape of what surrounds it.
///
/// Not a parser, but enough to give code the colour that makes it readable:
/// a name before `(` is being called, a capitalised one is a type, and one
/// before `=` or `:` at the start of a line is a key.
fn classify(
    chars: &[char],
    start: usize,
    end: usize,
    word: &str,
    syntax: &Syntax,
) -> Option<TokenKind> {
    if syntax.keywords.contains(&word) {
        return Some(TokenKind::Keyword);
    }

    let next = chars[end..].iter().find(|c| !c.is_whitespace()).copied();
    if next == Some('(') {
        return Some(TokenKind::Function);
    }

    let first_on_line = chars[..start].iter().all(|c| c.is_whitespace());
    if first_on_line && matches!(next, Some('=') | Some(':')) {
        return Some(TokenKind::Property);
    }

    if word.starts_with(char::is_uppercase) {
        return Some(TokenKind::Type);
    }
    None
}

fn starts_with(chars: &[char], needle: &str, at: usize) -> bool {
    let wanted: Vec<char> = needle.chars().collect();
    !wanted.is_empty()
        && chars.len() >= at + wanted.len()
        && chars[at..at + wanted.len()] == wanted[..]
}

fn find(chars: &[char], needle: &str, from: usize) -> Option<usize> {
    (from..chars.len()).find(|at| starts_with(chars, needle, *at))
}

/// The end of a quoted run, after the closing quote, honouring backslash
/// escapes. An unterminated string simply runs to the end of the line.
fn string_end(chars: &[char], start: usize, quote: char) -> usize {
    let mut at = start + 1;
    while at < chars.len() {
        if chars[at] == '\\' {
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
