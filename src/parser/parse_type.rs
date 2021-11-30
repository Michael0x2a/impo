use nom::branch::alt;
use nom::combinator::{map, map_opt, value};
use nom::multi::{many0, separated_list1};
use nom::sequence::{pair, preceded, terminated, tuple};
use struple::Struple;
use crate::ast::types::*;
use crate::ast::primitives::*;

use super::core::*;
use super::combinators::*;
use super::parse_expr::match_name;

pub fn match_type(tokens: &[Token]) -> ParseResult<TypeNode> {
    map_opt(
        separated_list1(
            TokenKind::Pipe,
            match_primary_type,
        ),
        |mut types| {
            if types.len() > 1 {
                Some(UnionType::new(types).into())
            } else {
                types.pop()
            }
        },
    )(tokens)
}

fn match_primary_type(tokens: &[Token]) -> ParseResult<TypeNode> {
    alt((
        match_reference_type,
        match_func_type,
        match_tuple_type,
        match_empty_type,
    ))(tokens)
}

fn match_reference_type(tokens: &[Token]) -> ParseResult<TypeNode> {
    map_into(
        pair(
            map(
                pair(
                    many0(terminated(match_name, TokenKind::Dot)),
                    match_name,
                ),
                Identifier::from_tuple,
            ),
            optional_delimited_list(
                TokenKind::LSquare,
                TokenKind::Comma,
                match_type,
                TokenKind::RSquare,
            ),
        ),
        ReferenceType::from_tuple,
    )(tokens)
}

fn match_func_type(tokens: &[Token]) -> ParseResult<TypeNode> {
    map_into(
        preceded(
            TokenKind::Fn,
            tuple((
                optional_delimited_list(
                    TokenKind::LSquare,
                    TokenKind::Comma,
                    match_name,
                    TokenKind::RSquare,
                ),
                delimited_list(
                    TokenKind::LParen,
                    TokenKind::Comma,
                    match_type,
                    TokenKind::RParen,
                ),
                opt_or(
                    preceded(
                        TokenKind::Arrow,
                        match_type,
                    ),
                    |o| o.unwrap_or(TypeNode::Unit),
                ),
            )),
        ),
        FuncType::from_tuple,
    )(tokens)
}

fn match_tuple_type(tokens: &[Token]) -> ParseResult<TypeNode> {
    map_into(
        delimited_list(
            TokenKind::LParen,
            TokenKind::Comma,
            match_type,
            TokenKind::RParen,
        ),
        |items| {
            if items.is_empty() {
                TypeNode::Unit
            } else {
                TupleType::new(items).into()
            }
        }
    )(tokens)
}

fn match_empty_type(tokens: &[Token]) -> ParseResult<TypeNode> {
    value(
        TypeNode::Empty,
        TokenKind::Bang,
    )(tokens)
}

#[cfg(test)]
mod tests {
    use nom;
    use super::*;
    use crate::parser::test_utils::*;

    #[test]
    fn test_reference_basic() -> Result<(), nom::Err<ParserError>> {
        let token_kinds = vec![
            atom("A"),
        ];

        parser_test(
            match_reference_type,
            &generate_positions(&token_kinds),
            ref_type_basic("A"),
        )
    }

    #[test]
    fn test_reference_with_lookup() -> Result<(), nom::Err<ParserError>> {
        let token_kinds = vec![
            atom("A"),
            TokenKind::Dot,
            atom("B"),
            TokenKind::Dot,
            atom("C"),
        ];

        parser_test(
            match_reference_type,
            &generate_positions(&token_kinds),
            ref_type_basic("A.B.C"),
        )
    }

    #[test]
    fn test_reference_with_generics() -> Result<(), nom::Err<ParserError>> {
        let token_kinds = vec![
            atom("A"),
            TokenKind::Dot,
            atom("B"),
            TokenKind::LSquare,
            atom("P1"),
            TokenKind::Dot,
            atom("P2"),
            TokenKind::Comma,
            atom("P3"),
            TokenKind::LSquare,
            atom("P4"),
            TokenKind::RSquare,
            TokenKind::RSquare,
        ];

        parser_test(
            match_reference_type,
            &generate_positions(&token_kinds),
            ref_type_generic("A.B", vec![
                ref_type_basic("P1.P2"),
                ref_type_generic("P3", vec![
                    ref_type_basic("P4"),
                ])
            ]),
        )
    }

    #[test]
    fn test_basic_function() -> Result<(), nom::Err<ParserError>> {
        let token_kinds = vec![
            TokenKind::Fn,
            TokenKind::LParen,
            TokenKind::RParen,
            TokenKind::Arrow,
            TokenKind::LParen,
            TokenKind::RParen,
        ];

        parser_test(
            match_func_type,
            &generate_positions(&token_kinds),
            FuncType{
                param_types: Vec::new(),
                typevars: Vec::new(),
                return_type: TypeNode::Unit,
            }.into(),
        )
    }

    #[test]
    fn test_basic_function_implicit_return() -> Result<(), nom::Err<ParserError>> {
        let token_kinds = vec![
            TokenKind::Fn,
            TokenKind::LParen,
            TokenKind::RParen,
        ];

        parser_test(
            match_func_type,
            &generate_positions(&token_kinds),
            FuncType{
                param_types: Vec::new(),
                typevars: Vec::new(),
                return_type: TypeNode::Unit,
            }.into(),
        )
    }

    #[test]
    fn test_func_with_params() -> Result<(), nom::Err<ParserError>> {
        let token_kinds = vec![
            TokenKind::Fn,
            TokenKind::LParen,
            atom("A"),
            TokenKind::Comma,
            atom("B"),
            TokenKind::RParen,
            TokenKind::Arrow,
            TokenKind::Bang,
        ];

        parser_test(
            match_func_type,
            &generate_positions(&token_kinds),
            FuncType{
                param_types: vec![
                    ref_type_basic("A"),
                    ref_type_basic("B"),
                ],
                typevars: Vec::new(),
                return_type: TypeNode::Empty,
            }.into(),
        )
    }

    #[test]
    fn test_func_with_generics() -> Result<(), nom::Err<ParserError>> {
        let token_kinds = vec![
            TokenKind::Fn,
            TokenKind::LSquare,
            atom("T1"),
            TokenKind::Comma,
            atom("T2"),
            TokenKind::RSquare,
            TokenKind::LParen,
            atom("A"),
            TokenKind::Comma,
            atom("B"),
            TokenKind::RParen,
        ];

        parser_test(
            match_func_type,
            &generate_positions(&token_kinds),
            FuncType{
                param_types: vec![
                    ref_type_basic("A"),
                    ref_type_basic("B"),
                ],
                typevars: vec![
                    "T1".into(),
                    "T2".into(),
                ],
                return_type: TypeNode::Unit,
            }.into(),
        )
    }

    #[test]
    fn test_nested_func() -> Result<(), nom::Err<ParserError>> {
        let token_kinds = vec![
            TokenKind::Fn,
            TokenKind::LParen,
            atom("A"),
            TokenKind::RParen,
            TokenKind::Arrow,
            TokenKind::Fn,
            TokenKind::LParen,
            atom("B"),
            TokenKind::RParen,
            TokenKind::Arrow,
            TokenKind::Fn,
            TokenKind::LParen,
            atom("C"),
            TokenKind::RParen,
        ];

        parser_test(
            match_func_type,
            &generate_positions(&token_kinds),
            FuncType{
                param_types: vec![
                    ref_type_basic("A"),
                ],
                typevars: Vec::new(),
                return_type: FuncType{
                    param_types: vec![
                        ref_type_basic("B"),
                    ],
                    typevars: Vec::new(),
                    return_type: FuncType{
                        param_types: vec![
                            ref_type_basic("C"),
                        ],
                        typevars: Vec::new(),
                        return_type: TypeNode::Unit,
                    }.into(),
                }.into(),
            }.into(),
        )
    }

    #[test]
    fn test_tuple() -> Result<(), nom::Err<ParserError>> {
        let token_kinds = vec![
            TokenKind::LParen,
            atom("A"),
            TokenKind::Comma,
            atom("B"),
            TokenKind::Comma,
            atom("C"),
            TokenKind::RParen,
        ];

        parser_test(
            match_tuple_type,
            &generate_positions(&token_kinds),
            TupleType{
                items: vec![
                    ref_type_basic("A"),
                    ref_type_basic("B"),
                    ref_type_basic("C"),
                ],
            }.into(),
        )
    }

    #[test]
    fn test_union_basic() -> Result<(), nom::Err<ParserError>> {
        let token_kinds = vec![
            atom("A"),
            TokenKind::Pipe,
            atom("B"),
            TokenKind::Pipe,
            atom("C"),
        ];

        parser_test(
            match_type,
            &generate_positions(&token_kinds),
            UnionType{
                variants: vec![
                    ref_type_basic("A"),
                    ref_type_basic("B"),
                    ref_type_basic("C"),
                ],
            }.into(),
        )
    }

    #[test]
    fn test_union_complex() -> Result<(), nom::Err<ParserError>> {
        let token_kinds = vec![
            TokenKind::Fn,
            TokenKind::LParen,
            atom("A"),
            TokenKind::RParen,
            TokenKind::Pipe,
            TokenKind::LParen,
            atom("A"),
            TokenKind::Comma,
            atom("B"),
            TokenKind::RParen,
            TokenKind::Pipe,
            atom("C"),
            TokenKind::Dot,
            atom("D"),
            TokenKind::Dot,
            atom("E"),
        ];

        parser_test(
            match_type,
            &generate_positions(&token_kinds),
            UnionType{
                variants: vec![
                    FuncType{
                        param_types: vec![
                            ref_type_basic("A"),
                        ],
                        typevars: Vec::new(),
                        return_type: TypeNode::Unit,
                    }.into(),
                    TupleType{
                        items: vec![
                            ref_type_basic("A"),
                            ref_type_basic("B"),
                        ],
                    }.into(),
                    ref_type_basic("C.D.E"),
                ],
            }.into(),
        )
    }
}