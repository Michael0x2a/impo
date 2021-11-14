use nom::combinator::{all_consuming, complete, map};
use nom::multi::many0;
use crate::ast::stmts::*;

use super::core::*;
use super::parse_expr::match_expr;

pub fn parse(tokens: &[Token]) -> Result<Vec<StmtNode>, nom::Err<ParserError>> {
    let (_, out) = all_consuming(complete(match_program))(tokens)?;
    Ok(out)
}

fn match_program(tokens: &[Token]) -> ParseResult<Vec<StmtNode>> {
    many0(match_stmt)(tokens)
}

fn match_stmt(tokens: &[Token]) -> ParseResult<StmtNode> {
    map(match_expr, StmtNode::Line)(tokens)
}