use nom::branch::alt;
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::{delimited, pair, preceded};

use crate::ast::exprs::*;
use crate::ast::primitives::*;

use super::core::*;

pub fn match_expr(tokens: &[Token]) -> ParseResult<ExprNode> {
    alt((match_lookup, match_variable, match_literal))(tokens)
}

fn match_group(tokens: &[Token]) -> ParseResult<ExprNode> {
    delimited(
        TokenKind::LParen,
        match_expr,
        TokenKind::RParen,
    )(tokens)
}

fn match_lookup(tokens: &[Token]) -> ParseResult<ExprNode> {
    let (rest, (parent, name_chain)) = pair(
         match_atom, 
        many1(preceded(
            TokenKind::Dot, 
            match_name,
        )),
    )(tokens)?;
    let expr = ExprNode::Lookup(Box::new(LookupExpr{
        source: parent,
        name_chain: name_chain,
    }));
    Ok((rest, expr))
}

fn match_atom(tokens: &[Token]) -> ParseResult<ExprNode> {
    // Add 'match group' -- use this to escape back to top?
    alt((match_variable, match_literal))(tokens)
}

fn match_variable(tokens: &[Token]) -> ParseResult<ExprNode> {
    map(match_name, ExprNode::Variable)(tokens)
}

fn match_name(tokens: &[Token]) -> ParseResult<Name> {
    let (rest, token) = get_next(tokens)?;
    match &token.kind {
        TokenKind::Atom(iden) => {
            Ok((rest, iden.clone()))
        },
        _ => {
            Err(err_bad_match("variable", token))
        }
    }
}

fn match_literal(tokens: &[Token]) -> ParseResult<ExprNode> {
    let (rest, token) = get_next(tokens)?;
    let output = match &token.kind {
        TokenKind::BoolLiteral(lit) => {
            ExprNode::BoolLiteral(*lit)
        },
        TokenKind::IntLiteral(lit) => {
            ExprNode::IntLiteral(Box::new(lit.clone()))
        },
        TokenKind::FloatLiteral(lit) => {
            ExprNode::FloatLiteral(Box::new(lit.clone()))
        },
        TokenKind::StringLiteral(lit) => {
            ExprNode::StringLiteral(Box::new(lit.clone()))
        },
        _ => {
            return Err(err_bad_match("literal", token));
        }
    };
    Ok((rest, output))
}

#[cfg(test)]
mod tests {
    use nom;
    use super::*;
    use crate::parser::test_utils::*;

    #[test]
    fn test_match_lookup() -> Result<(), nom::Err<ParserError>> {
        let token_kinds = vec![
            TokenKind::Atom("foo".into()),
            TokenKind::Dot,
            TokenKind::Atom("bar".into()),
            TokenKind::Dot,
            TokenKind::Atom("baz".into()),
        ];

        parser_test(
            match_lookup,
            &generate_positions(&token_kinds),
            ExprNode::Lookup(Box::new(LookupExpr{
                source: ExprNode::Variable(
                    "foo".into(),
                ),
                name_chain: vec![
                    "bar".into(),
                    "baz".into(),
                ],
            }))
        )
    }

    #[test]
    fn test_match_literal() -> Result<(), nom::Err<ParserError>> {
        let token_kinds = vec![
            TokenKind::IntLiteral(IntLiteral{
                base: 10,
                digits: "123".into(),
            }),
            TokenKind::FloatLiteral(FloatLiteral{
                integral_digits: "123".into(),
                fractional_digits: "567".into(),
                power: "".into(),
            }),
            TokenKind::StringLiteral("foo".into()),
            TokenKind::BoolLiteral(true),
        ];

        parser_test(
            nom::multi::many_m_n(4, 4, match_literal),
            &generate_positions(&token_kinds),
            vec![
                ExprNode::IntLiteral(Box::new(IntLiteral{
                    base: 10,
                    digits: "123".into(),
                })),
                ExprNode::FloatLiteral(Box::new(FloatLiteral{
                    integral_digits: "123".into(),
                    fractional_digits: "567".into(),
                    power: "".into(),
                })),
                ExprNode::StringLiteral(Box::new("foo".into())),
                ExprNode::BoolLiteral(true),
            ],
        )
    }
}