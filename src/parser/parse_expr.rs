use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::error::context;
use nom::multi::{many1, separated_list0};
use nom::sequence::{delimited, pair, preceded};

use crate::ast::exprs::*;
use crate::ast::primitives::*;

use super::core::*;
use super::combinators::*;

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

    fn merge(prev: (BP, ExprNode), op: InfixOp, next: (BP, ExprNode)) -> (BP, ExprNode) {
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

        if op == InfixOp::To {
            let range_infix = RangeExpr{
                start: prev_node,
                end: next_node,
            };
            return (next_bp, range_infix.into())
        }

        let new_infix = InfixExpr{
            exprs: vec![prev_node.clone(), next_node],
            ops: vec![op],
        };
        (next_bp, new_infix.into())
    }

    fn build(tokens: &[Token], min_bp: u8) -> ParseResult<ExprNode> {
        let mut rest = tokens;

        let (rest_prefix, op_prefix) = opt(match_prefix_op)(rest)?;

        let (rest_head, subexpr_head) = match op_prefix {
            Some(op) => {
                rest = rest_prefix;

                let ((), right_bp) = op.binding_power();
                let (rest_prefix_expr, subexpr_prefix_expr) = build(rest, right_bp)?;
                let curr = PrefixExpr{
                    op: op,
                    expr: subexpr_prefix_expr,
                };
                (rest_prefix_expr, curr.into())
            },
            None => match_operand(rest)?,
        };

        let mut curr = ((0, 0), subexpr_head);
        rest = rest_head;
        
        loop {
            let (rest_op, op) = opt(match_infix_op)(rest)?;

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
            rest = rest_tail;

            curr = merge(curr, op, ((left_bp, right_bp), subexpr_tail));
        }

        Ok((rest, curr.1))
    }

    context(
        "match_operations",
        |tokens| build(tokens, 0),
    )(tokens)
}

fn match_infix_op(tokens: &[Token]) -> ParseResult<InfixOp> {
    let (rest, token) = get_next(tokens, "infix op")?;
    let op = match &token.kind {
        TokenKind::Or => InfixOp::LogicalOr,
        TokenKind::And => InfixOp::LogicalAnd,

        TokenKind::Equals => InfixOp::Equals,
        TokenKind::NotEquals => InfixOp::NotEquals,
        TokenKind::LessThanEquals => InfixOp::LessThanEquals,
        TokenKind::GreaterThanEquals => InfixOp::GreaterThanEquals,
        TokenKind::LessThan => InfixOp::LessThan,
        TokenKind::GreaterThan => InfixOp::GreaterThan,
        TokenKind::To => InfixOp::To,

        TokenKind::InstanceOf => InfixOp::InstanceOf,

        TokenKind::Pipe => InfixOp::BitwiseOr,
        TokenKind::Caret => InfixOp::BitwiseXor,
        TokenKind::Ampersand => InfixOp::BitwiseAnd,

        TokenKind::ShiftLeft => InfixOp::BitwiseShiftLeft,
        TokenKind::ShiftRight => InfixOp::BitwiseShiftRight,

        TokenKind::Plus => InfixOp::Addition,
        TokenKind::Minus => InfixOp::Subtraction,

        TokenKind::Multiply => InfixOp::Multiplication,
        TokenKind::Divide => InfixOp::Division,
        TokenKind::Percent => InfixOp::Modulus,

        _ => {
            return Err(err_bad_match("infix operator", token))
        },
    };
    Ok((rest, op))
}

fn match_prefix_op(tokens: &[Token]) -> ParseResult<PrefixOp> {
    let (rest, token) = get_next(tokens, "prefix op")?;
    let op = match &token.kind {
        TokenKind::Bang=> PrefixOp::LogicalNegate,
        TokenKind::Minus => PrefixOp::NumericalNegate,
        TokenKind::Tilde => PrefixOp::BitwiseNegate,
        _ => {
            return Err(err_bad_match("prefix operator", token))
        },
    };
    Ok((rest, op))
}

// Operand -- anything that's a valid operand.
fn match_operand(tokens: &[Token]) -> ParseResult<ExprNode> {
    context(
        "match_operand", 
        alt((match_call_like, match_unit)),
    )(tokens)
}

fn match_call_like(tokens: &[Token]) -> ParseResult<ExprNode> {
    enum Tail {
        FuncCall(Vec<ExprNode>),
        Index(ExprNode),
    }

    fold1(
        match_unit,
        alt((
            map(
                delimited(
                    TokenKind::LParen,
                    separated_list0(
                        TokenKind::Comma,
                        match_expr,
                    ),
                    TokenKind::RParen,
                ),
                Tail::FuncCall,
            ),
            map(
            delimited(
                    TokenKind::LSquare,
                    match_expr,
                    TokenKind::RSquare,
                ),
                Tail::Index,
            )
        )),
        |curr, tail| {
            match tail {
                Tail::FuncCall(params) => FuncCallExpr{
                    func: curr,
                    params: params,
                }.into(),
                Tail::Index(index) => IndexExpr{
                    source: curr,
                    index: index,
                }.into(),
            }
        }
    )(tokens)
}

// Unit -- An entity that is either indivisible or consists of several
// pieces, where not all of the pieces are valid expressions.
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

// Atom -- a small, indivisible unit
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
                        ops: vec![InfixOp::Multiplication],
                    }.into(),
                ],
                ops: vec![InfixOp::Addition],
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