use std::fmt;
use std::cmp::{min, max};
use thiserror;
use nom::error as nom_error;

pub use crate::tokens::{Position, Token, TokenKind};

pub type ParseResult<'a, R> = nom::IResult<&'a [Token], R, ParserError>;

#[derive(thiserror::Error, Debug)]
pub struct ParserError {
    pub span: Option<(Position, Position)>,
    pub message: String,
    pub source: Option<Box<ParserError>>,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some((start, end)) = self.span {
            write!(f, "[{} - {}] {}", start, end, self.message)
        } else {
            write!(f, "[unknown pos] {}", self.message)
        }
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
    fn add_context(input: &[Token], ctx: &'static str, other: Self) -> Self {
        ParserError { 
            span: compute_span(input, other.span),
            message: ctx.to_owned(),
            source: Some(Box::new(other)),
        }
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

pub fn get_next(tokens: &[Token]) -> ParseResult<&Token> {
    let (token, rest) = tokens.split_first().ok_or_else(err_unexpected_eof)?;
    Ok((rest, token))
}

pub fn err_unexpected_eof() -> nom::Err<ParserError> {
    nom::Err::Error(ParserError{
        span: None,
        message: "Unexpected EOF".to_owned(),
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
        let (rest, token) = get_next(tokens)?;
        if &token.kind == self {
            Ok((rest, token))
        } else {
            Err(err_bad_match(self.name(), token))
        }
    }
}

#[allow(dead_code)]
pub fn debug<I, T, E>(message: &'static str, parser: impl nom::Parser<I, T, E>) -> impl nom::Parser<I, T, E>
where T: std::fmt::Debug
{
    nom::combinator::map(parser, move |found| {
        println!("{} produced {:?}", message, found);
        found
    })
}