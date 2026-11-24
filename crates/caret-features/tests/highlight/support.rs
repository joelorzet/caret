use caret_features::{HighlightState, Language, Token, TokenKind};

pub fn tokens(language: Language, line: &str) -> Vec<Token> {
    language.highlight(line, &mut HighlightState::default())
}

pub fn kinds(language: Language, line: &str) -> Vec<TokenKind> {
    tokens(language, line).into_iter().map(|t| t.kind).collect()
}

pub fn text(line: &str, token: Token) -> String {
    line.chars()
        .skip(token.start)
        .take(token.end - token.start)
        .collect()
}

pub fn kind_of(language: Language, line: &str, word: &str) -> Option<TokenKind> {
    let start = line.find(word)?;
    let start = line[..start].chars().count();
    tokens(language, line)
        .into_iter()
        .find(|t| t.start == start)
        .map(|t| t.kind)
}
