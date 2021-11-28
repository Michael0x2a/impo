use nom::branch::{alt};
use nom::combinator::{complete, map};
use nom::multi::{many0, fold_many0};
use nom::sequence::{pair, terminated};
use crate::ast::stmts::*;

use super::core::*;
use super::parse_expr::match_expr;

pub fn parse(tokens: &[Token]) -> Result<Program, nom::Err<ParserError>> {
    let (rest, out) = complete(match_program)(tokens)?;
    rest.first().map_or(
        Ok(out),
        |extra_token| Err(err_unexpected_token(extra_token)),
    )
}

fn match_program(tokens: &[Token]) -> ParseResult<Program> {
    map(
        many0(match_stmt),
        |body| Program{body: body},
    )(tokens)
}

fn match_stmt(tokens: &[Token]) -> ParseResult<StmtNode> {
    alt((
        match_line,
        match_empty_line,
    ))(tokens)
}

fn match_line(tokens: &[Token]) -> ParseResult<StmtNode> {
    map(
        terminated(
            pair(match_comment, match_expr),
            TokenKind::Newline,
        ),
        |(comment, expr)| LineStmt{
            comment: comment,
            expr: expr,
        }.into(),
    )(tokens)
}

fn match_empty_line(token: &[Token]) -> ParseResult<StmtNode> {
    map(TokenKind::Newline, |_| StmtNode::EmptyLine())(token)
}

fn match_comment(tokens: &[Token]) -> ParseResult<Comment> {
    fn match_single_comment(tokens: &[Token]) -> ParseResult<String> {
        let (rest, token) = get_next(tokens, "comment")?;
        let output = match &token.kind {
            TokenKind::Comment(body) => body.clone(),
            _ => {
                return Err(err_bad_match("comment", token));
            }
        };
        Ok((rest, output))
    }

    map(
        fold_many0(
            terminated(match_single_comment, TokenKind::Newline),
            Vec::new,
            |mut arr, line| {
                arr.push(line);
                arr
            },
        ),
        Comment::new,
    )(tokens)
}