use crate::support::{kinds, tokens};
use caret_features::{Language, TokenKind};
use std::path::Path;

#[test]
fn the_language_is_guessed_from_the_name() {
    assert_eq!(Language::of(Path::new("main.rs")), Language::Rust);
    assert_eq!(Language::of(Path::new("a/b/script.py")), Language::Python);
    assert_eq!(Language::of(Path::new("app.tsx")), Language::JavaScript);
    assert_eq!(Language::of(Path::new("Cargo.toml")), Language::Toml);
    assert_eq!(Language::of(Path::new("Cargo.lock")), Language::Toml);
    assert_eq!(Language::of(Path::new("Dockerfile")), Language::Shell);
    assert_eq!(Language::of(Path::new("README.md")), Language::Markdown);
    assert_eq!(Language::of(Path::new("mystery")), Language::Plain);
}

#[test]
fn shell_and_python_use_hash_comments() {
    for language in [
        Language::Shell,
        Language::Python,
        Language::Toml,
        Language::Yaml,
    ] {
        let kinds = kinds(language, "value # a note");
        assert!(kinds.contains(&TokenKind::Comment), "{language:?}");
    }
    // In Rust `#` starts an attribute, not a comment.
    assert!(!kinds(Language::Rust, "#[derive(Debug)]").contains(&TokenKind::Comment));
}

#[test]
fn plain_text_is_left_entirely_alone() {
    assert!(tokens(Language::Plain, "fn main() { \"x\" } // 42").is_empty());
}
