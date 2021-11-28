use nom::branch::{alt};
use nom::combinator::{complete, map, opt, value};
use nom::multi::{many0, many1, fold_many0};
use nom::sequence::{pair, terminated, tuple};
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

fn match_block(tokens: &[Token]) -> ParseResult<Block> {
    many1(match_stmt)(tokens)
}

fn match_stmt(tokens: &[Token]) -> ParseResult<StmtNode> {
    alt((
        match_if,
        match_line,
        match_empty_line,
    ))(tokens)
}

fn match_if(tokens: &[Token]) -> ParseResult<StmtNode> {
    super::combinators::print_tokens("match_if", 40, tokens);

    let match_if = map(
        tuple((
            TokenKind::If,
            match_expr,
            TokenKind::Colon,
            TokenKind::Newline,
            TokenKind::Indent,
            match_block,
            TokenKind::Unindent,
        )),
        |(_, cond, _, _, _, body, _)| (cond, body),
    );
    let match_elif = map(
        tuple((
            TokenKind::Elif,
            match_expr,
            TokenKind::Colon,
            TokenKind::Newline,
            TokenKind::Indent,
            match_block,
            TokenKind::Unindent,
        )),
        |(_, cond, _, _, _, body, _)| (cond, body),
    );
    let match_else = map(
        tuple((
            TokenKind::Else,
            TokenKind::Colon,
            TokenKind::Newline,
            TokenKind::Indent,
            match_block,
            TokenKind::Unindent,
        )),
        |(_, _, _, _, body, _)| body,
    );

    map(
        tuple((
            match_comment,
            match_if,
            many0(match_elif),
            opt(match_else),   
        )),
        |(comment, if_branch, elif_branches, else_branch)| IfStmt{
            comment: comment,
            if_branch: if_branch,
            elif_branches: elif_branches,
            else_branch: else_branch,
        }.into()
    )(tokens)
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
    value(StmtNode::EmptyLine(), TokenKind::Newline)(token)
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