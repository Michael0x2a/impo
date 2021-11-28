use std::fmt;
use std::cmp::{min, max};
use nom::error as nom_error;

pub use crate::tokens::{Position, Token, TokenKind};

pub type ParseResult<'a, R> = nom::IResult<&'a [Token], R, ParserError>;

#[derive(thiserror::Error)]
pub struct ParserError {
    pub span: Option<(Position, Position)>,
    pub message: String,
    pub source: Option<Box<ParserError>>,
}

impl ParserError {
    fn error_lines(&self) -> Vec<String> {
        let mut out = Vec::new();
        let mut curr = self;
        loop {
            let line = if let Some((start, end)) = curr.span {
                format!("[{} - {}] {}", start, end, curr.message)
            } else {
                format!("[unknown pos] {}", curr.message)
            };
            out.push(line);

            if let Some(next) = &curr.source {
                curr = next;
            } else {
                break;
            }
        }
        out
    }

    fn add_context(self, ctx: &'static str) -> Self {
        ParserError {
            span: self.span,
            message: format!("{}: {}", ctx, self.message),
            source: self.source,
        }
    }
}

impl fmt::Debug for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        for line in self.error_lines() {
            writeln!(f, "    {}", line)?;
        }
        Ok(())
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error_lines().join("\n"))
    }
}

impl nom_error::ParseError<&[Token]> for ParserError {
    fn from_error_kind(input: &[Token], kind: nom_error::ErrorKind) -> Self {
        ParserError { 
            span: input.get(0).map(Token::span),
            message: format!("error from {}", kind.description()),
            source: None,
        }
    }

    fn append(input: &[Token], kind: nom_error::ErrorKind, other: Self) -> Self {
        ParserError { 
            span: compute_span(input, other.span),
            message: format!("error from {}", kind.description()),
            source: Some(Box::new(other)),
        }
    }
}

impl nom_error::ContextError<&[Token]> for ParserError {
    fn add_context(_input: &[Token], ctx: &'static str, other: Self) -> Self {
        other.add_context(ctx)
    }
}

fn compute_span(input: &[Token], existing_span: Option<(Position, Position)>) -> Option<(Position, Position)> {
    input.get(0)
        .map(Token::span)
        .map(|(start, end)| {
            if let Some((other_start, other_end)) = existing_span {
                (min(start, other_start), max(end, other_end))
            } else {
                (start, end)
            }
        })
}

pub fn get_next(
    tokens: &[Token],
    target: impl AsRef<str>,
) -> ParseResult<&Token> {
    let (token, rest) = tokens
        .split_first()
        .ok_or_else(|| err_unexpected_eof(target))?;
    Ok((rest, token))
}

pub fn err_unexpected_eof(target: impl AsRef<str>) -> nom::Err<ParserError> {
    nom::Err::Error(ParserError{
        span: None,
        message: format!("Unexpected EOF, looking for {}", target.as_ref()),
        source: None,
    })
}

pub fn err_unexpected_token(token: &Token) -> nom::Err<ParserError> {
    nom::Err::Error(ParserError{
        span: Some(token.span()),
        message: format!("Unexpected token '{}', expected EOF", token.kind.name()),
        source: None,
    })
}


pub fn err_bad_match(expected: &str, actual: &Token) -> nom::Err<ParserError> {
    nom::Err::Error(ParserError{
        span: Some(actual.span()),
        message: format!("Expected {}, got {}", expected, actual.kind.name()),
        source: None,
    })
}

impl<'a> nom::Parser<&'a [Token], &'a Token, ParserError> for TokenKind {
    fn parse(&mut self, tokens: &'a [Token]) -> ParseResult<'a, &'a Token> {
        let (rest, token) = get_next(tokens, self.name())?;
        if &token.kind == self {
            Ok((rest, token))
        } else {
            Err(err_bad_match(self.name(), token))
        }
    }
}
