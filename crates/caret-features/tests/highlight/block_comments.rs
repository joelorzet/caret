use crate::support::text;
use caret_features::{HighlightState, Language, TokenKind};

// A block comment spans lines, so the state has to survive between them.
#[test]
fn block_comments_carry_across_lines() {
    let mut state = HighlightState::default();

    let first = Language::Rust.highlight("code /* start", &mut state);
    assert_eq!(first.last().map(|t| t.kind), Some(TokenKind::Comment));
    assert!(
        state.is_inside_block_comment(),
        "still inside after the first line"
    );

    let middle = Language::Rust.highlight("everything here", &mut state);
    assert_eq!(middle.len(), 1);
    assert_eq!(middle[0].kind, TokenKind::Comment);
    assert!(state.is_inside_block_comment());

    let last = Language::Rust.highlight("*/ let x = 1;", &mut state);
    assert!(!state.is_inside_block_comment(), "closed");
    assert_eq!(last[0].kind, TokenKind::Comment);
    assert!(
        last.iter().any(|t| t.kind == TokenKind::Keyword),
        "code after it resumes"
    );
}

#[test]
fn a_block_comment_opened_and_closed_on_one_line_does_not_leak() {
    let mut state = HighlightState::default();
    let line = "let /* aside */ x = 1;";
    let found = Language::Rust.highlight(line, &mut state);
    assert!(!state.is_inside_block_comment());
    assert_eq!(
        found
            .iter()
            .filter(|t| t.kind == TokenKind::Comment)
            .map(|t| text(line, *t))
            .collect::<Vec<_>>(),
        ["/* aside */"]
    );
}
