#[allow(unused_imports)]
use crate::prelude::*;

/// Parses the layout spec language.
///
/// ```text
/// node   := ('h' | 'v') '(' child (',' child)* ')' | '*'
/// child  := [percent ':'] node
/// ```
///
/// `*` is a pane. A child without a percentage takes an equal share of
/// whatever the explicit ones leave, so `h(*, v(*, *))` is a 50/50 split with
/// the right half stacked, and `h(60:*, 40:v(*, *))` spells the same shape
/// out.
impl LayoutNode {
    pub fn parse(spec: &str) -> Result<LayoutNode, LayoutSpecError> {
        let bytes = spec.as_bytes();
        let mut p = Parser {
            input: spec,
            bytes,
            at: 0,
        };
        let node = p.node()?;
        p.skip_space();
        if p.at < bytes.len() {
            return Err(p.err("unexpected trailing input"));
        }
        Ok(node)
    }
}

struct Parser<'a> {
    input: &'a str,
    bytes: &'a [u8],
    at: usize,
}

impl Parser<'_> {
    fn err(&self, reason: &'static str) -> LayoutSpecError {
        LayoutSpecError {
            input: self.input.to_string(),
            reason,
            at: self.at,
        }
    }

    fn skip_space(&mut self) {
        while self.at < self.bytes.len() && self.bytes[self.at].is_ascii_whitespace() {
            self.at += 1;
        }
    }

    fn peek(&mut self) -> Option<u8> {
        self.skip_space();
        self.bytes.get(self.at).copied()
    }

    fn node(&mut self) -> Result<LayoutNode, LayoutSpecError> {
        match self.peek() {
            Some(b'*') => {
                self.at += 1;
                Ok(LayoutNode::pane())
            }
            Some(c @ (b'h' | b'H' | b'v' | b'V')) => {
                self.at += 1;
                let axis = if c.eq_ignore_ascii_case(&b'h') {
                    Axis::Horizontal
                } else {
                    Axis::Vertical
                };
                self.expect(b'(')?;
                let mut children = vec![self.child()?];
                while self.peek() == Some(b',') {
                    self.at += 1;
                    children.push(self.child()?);
                }
                self.expect(b')')?;
                if children.len() < 2 {
                    return Err(self.err("a split needs at least two children"));
                }
                Ok(LayoutNode::split(axis, children))
            }
            Some(_) => Err(self.err("expected '*', 'h' or 'v'")),
            None => Err(self.err("unexpected end of spec")),
        }
    }

    fn child(&mut self) -> Result<LayoutNode, LayoutSpecError> {
        let weight = self.weight()?;
        let node = self.node()?;
        Ok(match weight {
            Some(w) => node.weighted(w),
            None => node,
        })
    }

    /// A leading `number ':'`, if present.
    fn weight(&mut self) -> Result<Option<f32>, LayoutSpecError> {
        self.skip_space();
        let start = self.at;
        while self
            .bytes
            .get(self.at)
            .is_some_and(|c| c.is_ascii_digit() || *c == b'.')
        {
            self.at += 1;
        }
        if start == self.at {
            return Ok(None);
        }
        let digits = &self.input[start..self.at];
        if self.peek() != Some(b':') {
            self.at = start;
            return Err(self.err("a percentage must be followed by ':'"));
        }
        self.at += 1;
        match digits.parse::<f32>() {
            Ok(w) if w > 0.0 && w <= 100.0 => Ok(Some(w)),
            _ => Err(self.err("a percentage must be between 0 and 100")),
        }
    }

    fn expect(&mut self, want: u8) -> Result<(), LayoutSpecError> {
        if self.peek() == Some(want) {
            self.at += 1;
            Ok(())
        } else if want == b'(' {
            Err(self.err("expected '('"))
        } else {
            Err(self.err("expected ')'"))
        }
    }
}

impl core::fmt::Display for LayoutSpecError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "invalid layout {:?}: {} at position {}",
            self.input, self.reason, self.at
        )
    }
}

impl std::error::Error for LayoutSpecError {}
