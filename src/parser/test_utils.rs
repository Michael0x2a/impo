use nom::combinator::{all_consuming, complete};
use crate::ast::ExprNode;

pub use crate::tokens::{Token, TokenKind, Position};
pub use crate::values::{IntLiteral, FloatLiteral};
pub use super::core::*;

pub fn parser_test<'a, T: Eq + std::fmt::Debug>(
    parser: impl nom::Parser<&'a [Token], T, ParserError>,
    tokens: &'a [Token],
    expected: T,
) -> Result<(), nom::Err<ParserError>> {
    let mut full_parser = complete(all_consuming(parser));
    
    let (rest, output) = full_parser(&tokens)?;
    assert!(rest.is_empty());
    assert_eq!(output, expected);

    Ok(())
}

pub fn generate_positions(kinds: &[TokenKind]) -> Vec<Token> {
    kinds.iter()
        .enumerate()
        .map(|(i, token_kind)| {
            Token{
                kind: token_kind.clone(),
                position: Position::new(1, 0, i),
            }
        })
        .collect()
}

pub fn atom(name: &'static str) -> TokenKind {
    TokenKind::Atom(name.into())
}

pub fn variable(name: &'static str) -> ExprNode {
    ExprNode::Variable(name.into())
}