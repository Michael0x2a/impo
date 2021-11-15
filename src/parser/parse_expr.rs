use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::error::context;
use nom::multi::many1;
use nom::sequence::{delimited, pair, preceded};

use crate::ast::exprs::*;
use crate::ast::primitives::*;

use super::core::*;

pub fn match_expr(tokens: &[Token]) -> ParseResult<ExprNode> {
    context(
        "match_expr", 
        alt((match_operations, match_group)),
    )(tokens)
}

fn match_group(tokens: &[Token]) -> ParseResult<ExprNode> {
    context(
        "match_group",
        map(
            delimited(
                TokenKind::LParen,
                match_expr,
                TokenKind::RParen,
            ),
            |e| ExprNode::ExplicitParenthesis(Box::new(e)),
        ),
    )(tokens)
}

fn match_operations(tokens: &[Token]) -> ParseResult<ExprNode> {
    type BP = (u8, u8);

    fn merge(prev: (BP, ExprNode), op: Operator, next: (BP, ExprNode)) -> (BP, ExprNode) {
        let (prev_bp, prev_node) = prev;
        let (next_bp, next_node) = next;

        if prev_bp == next_bp {
            if let ExprNode::Infix(infix) = prev_node {
                let mut new_infix = *infix;
                new_infix.exprs.push(next_node);
                new_infix.ops.push(op);
                return (prev_bp, new_infix.into())
            }
        }

        let new_infix = InfixExpr{
            exprs: vec![prev_node.clone(), next_node],
            ops: vec![op],
        };
        (next_bp, new_infix.into())
    }

    fn build(tokens: &[Token], min_bp: u8) -> ParseResult<ExprNode> {
        let mut rest = tokens;

        let (rest_head, subexpr_head) = match_unit(rest)?;

        let mut curr = ((0, 0), subexpr_head);
        rest = rest_head;
        
        loop {
            let (rest_op, op) = opt(match_binary_op)(rest)?;

            let op = match op {
                Some(op) => op,
                None => { break; }
            };

            let (left_bp, right_bp) = op.binding_power();
            if left_bp < min_bp {
                break;
            }

            rest = rest_op;
            let (rest_tail, subexpr_tail) = build(rest, right_bp)?;

            curr = merge(curr, op, ((left_bp, right_bp), subexpr_tail));
            rest = rest_tail;
        }

        Ok((rest, curr.1))
    }

    context(
        "match_operations",
        |tokens| build(tokens, 0),
    )(tokens)
}

fn match_binary_op(tokens: &[Token]) -> ParseResult<Operator> {
    let (rest, token) = get_next(tokens, "operator")?;
    let op = match &token.kind {
        TokenKind::Plus => Operator::Addition,
        TokenKind::Minus => Operator::Subtraction,
        TokenKind::Multiply => Operator::Multiplication,
        TokenKind::Divide => Operator::Division,
        TokenKind::Percent => Operator::Modulus,
        TokenKind::Equals => Operator::Equals,
        TokenKind::NotEquals => Operator::NotEquals,
        TokenKind::LessThanEquals => Operator::LessThanEquals,
        TokenKind::GreaterThanEquals => Operator::GreaterThanEquals,
        TokenKind::LessThan => Operator::LessThan,
        TokenKind::GreaterThan => Operator::GreaterThan,
        TokenKind::Pipe => Operator::BitwiseOr,
        TokenKind::Ampersand => Operator::BitwiseAnd,
        TokenKind::Or => Operator::LogicalOr,
        TokenKind::And => Operator::LogicalAnd,
        TokenKind::InstanceOf => Operator::InstanceOf,
        _ => {
            return Err(err_bad_match("operator", token))
        },
    };
    Ok((rest, op))
}

fn match_unit(tokens: &[Token]) -> ParseResult<ExprNode> {
    context(
        "match_unit", 
        alt((match_lookup, match_atom)),
    )(tokens)
}

fn match_lookup(tokens: &[Token]) -> ParseResult<ExprNode> {
    let (rest, (parent, name_chain)) = context(
        "match_lookup",
        pair(
            match_atom, 
            many1(preceded(
                TokenKind::Dot, 
                match_name,
            )),
        )
    )(tokens)?;
    let expr = LookupExpr{
        source: parent,
        name_chain: name_chain,
    };
    Ok((rest, expr.into()))
}

fn match_atom(tokens: &[Token]) -> ParseResult<ExprNode> {
    context(
        "match_atom",
    alt((match_variable, match_literal, match_group)),
    )(tokens)
}

fn match_variable(tokens: &[Token]) -> ParseResult<ExprNode> {
    map(match_name, ExprNode::Variable)(tokens)
}

fn match_name(tokens: &[Token]) -> ParseResult<Name> {
    let (rest, token) = get_next(tokens, "name")?;
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
    let (rest, token) = get_next(tokens, "literal")?;
    let output = match &token.kind {
        TokenKind::BoolLiteral(lit) => ExprNode::BoolLiteral(*lit),
        TokenKind::IntLiteral(lit) => lit.clone().into(),
        TokenKind::FloatLiteral(lit) => lit.clone().into(),
        TokenKind::StringLiteral(lit) => lit.clone().into(),
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
    fn test_operators() -> Result<(), nom::Err<ParserError>> {
        let token_kinds = vec![
            atom("a"),
            TokenKind::Plus,
            atom("b"),
            TokenKind::Multiply,
            atom("c"),
        ];

        parser_test(
            match_operations,
            &generate_positions(&token_kinds),
            InfixExpr{
                exprs: vec![
                    variable("a"),
                    InfixExpr{
                        exprs: vec![
                            variable("b"),
                            variable("c"),
                        ],
                        ops: vec![Operator::Multiplication],
                    }.into(),
                ],
                ops: vec![Operator::Addition],
            }.into(),
        )
    }

    #[test]
    fn test_match_lookup() -> Result<(), nom::Err<ParserError>> {
        let token_kinds = vec![
            atom("foo"),
            TokenKind::Dot,
            atom("bar"),
            TokenKind::Dot,
            atom("baz"),
        ];
        parser_test(
            match_lookup,
            &generate_positions(&token_kinds),
            LookupExpr{
                source: variable("foo"),
                name_chain: vec![
                    "bar".into(),
                    "baz".into(),
                ],
            }.into(),
        )
    }

    #[test]
    fn test_match_lookup_nested() -> Result<(), nom::Err<ParserError>> {
        let token_kinds = vec![
            TokenKind::LParen,
            atom("foo"),
            TokenKind::Dot,
            atom("bar"),
            TokenKind::RParen,
            TokenKind::Dot,
            atom("baz"),
            TokenKind::Dot,
            atom("qux"),
        ];
        parser_test(
            match_lookup,
            &generate_positions(&token_kinds),
            LookupExpr{
                source: ExprNode::ExplicitParenthesis(Box::new(
                    LookupExpr{
                        source: variable("foo"),
                        name_chain: vec![
                            "bar".into(),
                        ],
                    }.into(),
                )),
                name_chain: vec![
                    "baz".into(),
                    "qux".into(),
                ],
            }.into(),
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
                IntLiteral{
                    base: 10,
                    digits: "123".into(),
                }.into(),
                FloatLiteral{
                    integral_digits: "123".into(),
                    fractional_digits: "567".into(),
                    power: "".into(),
                }.into(),
                "foo".into(),
                ExprNode::BoolLiteral(true),
            ],
        )
    }
}