use nom::branch::{alt};
use nom::combinator::{complete, map, opt, value};
use nom::multi::{many0, many1, fold_many0};
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
use struple::Struple;

use crate::ast::{FuncType, stmts::*};
use crate::ast::types::TypeNode;

use super::core::*;
use super::combinators::*;
use super::parse_expr::{match_expr, match_name};
use super::parse_type::match_type;

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
        match_func_signature_def,
        match_func_implementation_def,
        match_if,
        match_assignment,
        match_line,
        match_return,
        match_panic,
        match_empty_line,
    ))(tokens)
}

fn match_func_signature_def(tokens: &[Token]) -> ParseResult<StmtNode> {
    map(
        terminated(match_func_header, TokenKind::Newline),
        FuncSignatureDefStmt::into,
    )(tokens)
}

fn match_func_implementation_def(tokens: &[Token]) -> ParseResult<StmtNode> {
    map_into(
        tuple((
            match_func_header,
            TokenKind::Colon,
            TokenKind::Newline,
            TokenKind::Indent,
            match_block,
            TokenKind::Unindent,
        )),
        |(function, _, _, _, body, _)| FuncImplementationDefStmt{
            function: function,
            body: body,
        }
    )(tokens)
}

fn match_func_header(tokens: &[Token]) -> ParseResult<FuncSignatureDefStmt> {
    let match_type_vars = optional_delimited_list(
        TokenKind::LSquare,
        TokenKind::Comma,
        match_name,
        TokenKind::RSquare,
    );
    let match_params = delimited_list(
        TokenKind::LParen,
        TokenKind::Comma,
        separated_pair(
            match_name,
            TokenKind::Colon,
            match_type,
        ),
        TokenKind::RParen,
    );
    let match_return = opt_or(
        preceded(
            TokenKind::Arrow,
            match_type,
        ),
        |o| o.unwrap_or(TypeNode::Unit),
    );
    map(
        tuple((
            match_comment,
            preceded(TokenKind::Fn, match_name),
            match_type_vars,
            match_params,
            match_return,
        )),
        |(comment, name, typevars, params, return_type)| {
            let (param_names, param_types) = params.into_iter().unzip();
            FuncSignatureDefStmt{
                comment: comment,
                name: name,
                signature: FuncType{
                    typevars: typevars,
                    param_types: param_types,
                    return_type: return_type,
                },
                param_names: param_names,
            }
        }
    )(tokens)
}

fn match_if(tokens: &[Token]) -> ParseResult<StmtNode> {
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

    map_into(
        tuple((
            match_comment,
            match_if,
            many0(match_elif),
            opt(match_else),   
        )),
        IfStmt::from_tuple,
    )(tokens)
}

fn match_assignment(tokens: &[Token]) -> ParseResult<StmtNode> {
    map_into(
        terminated(
            tuple((
                match_comment,
                match_expr,
                TokenKind::Assign,
                match_expr,
            )),
            TokenKind::Newline,
        ),
        |(comment, left, _, right)| {
            AssignmentStmt::from_tuple((comment, left, right))
        },
    )(tokens)
}

fn match_return(tokens: &[Token]) -> ParseResult<StmtNode> {
    map_into(
        pair(
            match_comment,
            delimited(TokenKind::Return, opt(match_expr), TokenKind::Newline),
        ),
        ReturnStmt::from_tuple,
    )(tokens)
}

fn match_panic(tokens: &[Token]) -> ParseResult<StmtNode> {
    map_into(
        pair(
            match_comment,
            delimited(TokenKind::Panic, opt(match_expr), TokenKind::Newline),
        ),
        ReturnStmt::from_tuple,
    )(tokens)
}

fn match_line(tokens: &[Token]) -> ParseResult<StmtNode> {
    map_into(
        terminated(
            pair(match_comment, match_expr),
            TokenKind::Newline,
        ),
        LineStmt::from_tuple,
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