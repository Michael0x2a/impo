use itertools::Itertools;
use crate::ast::*;
use crate::string_utils::StringJoinExt;

#[must_use]
pub fn prettyprint_program(program: Program) -> String {
    prettyprint_stmt(&program.into())
}

#[allow(clippy::too_many_lines)]
fn prettyprint_stmt(stmt: &StmtNode) -> String {
    fn with_indent(level: usize, text: impl AsRef<str>) -> String {
        format!("{}{}", " ".repeat(level), text.as_ref())
    }

    type Writer<'a> = Box<dyn Fn(&mut Vec<String>, usize) + 'a>;

    fn sequence(children: Vec<Writer>) -> Writer {
        Box::new(move |lines, level| {
            for child_writer in &children {
                child_writer(lines, level);
            }
        })
    }

    // Technically redundant with sequence', but this one avoids the extra Vec allocation
    fn pair<'a>(writer1: Writer<'a>, writer2: Writer<'a>) -> Writer<'a> {
        Box::new(move |lines, level| {
            writer1(lines, level);
            writer2(lines, level);
        })
    }

    fn with_comment<'a>(c: &'a Comment, writer: Writer<'a>) -> Writer<'a> {
        sequence(vec![comment(c), writer])
    }

    fn empty<'a>() -> Writer<'a> {
        Box::new(move |lines, _| {
            lines.push("".to_owned());
        })
    }

    fn literal<'a>(text: String) -> Writer<'a> {
        Box::new(move |lines, level| {
            lines.push(with_indent(level, &text));
        })
    }

    fn comment(comment: &Comment) -> Writer {
        sequence(comment.lines.iter().map(|line| format!("# {}", line)).map(literal).collect())
    }

    fn bare_block<'a>(
        name: &'a str,
        body: Writer<'a>,
    ) -> Writer<'a> {
        Box::new(move |lines, level| {
            lines.push(with_indent(level, format!("({}", name)));
            body(lines, level + 4);
            lines.push(with_indent(level, ")"));
        })
    }

    fn expr_block<'a>(
        name: &'a str,
        exprs: Vec<String>,
        body: Writer<'a>,
    ) -> Writer<'a> {
        Box::new(move |lines, level| {
            lines.push(with_indent(level, format!("({} {}", name, exprs.join(" "))));
            body(lines, level + 4);
            lines.push(with_indent(level, ")"));
        })
    }

    fn line<'a>(prefix: &'static str, expr: &'a ExprNode) -> Writer<'a> {
        Box::new(move |lines, level| {
            let indent = " ".repeat(level);
            let text = prettyprint_expr(expr);
            if prefix.is_empty() {
                lines.push(format!("{}{}", indent, text));
            } else {
                lines.push(format!("{}{}{}", indent, prefix, text));
            }
        })
    }

    fn write_block(block: &[StmtNode]) -> Writer {
        Box::new(move |lines, level| {
            for stmt in block {
                write_stmt(stmt)(lines, level);
            }
        })
    }

    fn write_stmt(stmt: &StmtNode) -> Writer {
        match stmt {
            StmtNode::Program(s) => {
                write_block(&s.body)
            },
            StmtNode::Import(s) => {
                with_comment(
                    &s.comment,
                    bare_block("import", pair(
                        literal(s.source.to_string()),
                        literal(s.imports.iter().map(quote).join(" ")),
                    )),
                )
            },
            StmtNode::InterfaceDef(s) => {
                with_comment(
                    &s.comment,
                    bare_block("interface", sequence(vec![
                        literal("WIP".to_owned()),
                    ])),
                )
            },
            StmtNode::ClassDef(s) => {
                with_comment(
                    &s.comment,
                    bare_block("class", sequence(vec![
                        literal("WIP".to_owned()),
                    ])),
                )
            },
            StmtNode::SentinalDef(s) => {
                with_comment(
                    &s.comment,
                    bare_block("sentinal", sequence(vec![
                        literal("WIP".to_owned()),
                    ])),
                )
            },
            StmtNode::FieldSignatureDef(s) => {
                with_comment(
                    &s.comment,
                    bare_block("field", sequence(vec![
                        literal("WIP".to_owned()),
                    ])),
                )
            },
            StmtNode::FuncSignatureDef(s) => {
                with_comment(
                    &s.comment,
                    bare_block("fn", sequence(vec![
                        literal("WIP".to_owned()),
                    ]))
                )
            },
            StmtNode::FuncImplementationDef(s) => {
                with_comment(
                    &s.comment,
                    bare_block("fn", sequence(vec![
                        literal("WIP".to_owned()),
                    ])),
                )
            },
            StmtNode::If(s) => {
                let mut writers = vec![expr_block(
                    "if-branch",
                    vec![prettyprint_expr(&s.if_branch.0)], 
                    write_block(&s.if_branch.1),
                )];
                for (cond, body) in &s.elif_branches {
                    writers.push(expr_block(
                        "elif-branch",
                        vec![prettyprint_expr(cond)], 
                        write_block(body),
                    ));
                }
                if let Some(body) = &s.else_branch {
                    writers.push(bare_block(
                        "else-branch",
                        write_block(body),
                    ));
                }
                sequence(vec![
                    comment(&s.comment),
                    bare_block("if", sequence(writers)),
                ])
            },
            StmtNode::For(s) => {
                with_comment(
                    &s.comment,
                    bare_block("for", sequence(vec![
                        literal("WIP".to_owned()),
                    ])),
                )
            },
            StmtNode::Foreach(s) => {
                with_comment(
                    &s.comment,
                    bare_block("foreach", sequence(vec![
                        literal("WIP".to_owned()),
                    ])),
                )
            },
            StmtNode::While(s) => {
                with_comment(
                    &s.comment,
                    bare_block("while", sequence(vec![
                        literal("WIP".to_owned()),
                    ])),
                )
            },
            StmtNode::Return(s) => {
                with_comment(
                    &s.comment,
                    bare_block("return", sequence(vec![
                        literal("WIP".to_owned()),
                    ])),
                )
            },
            StmtNode::Panic(s) => {
                with_comment(
                    &s.comment,
                    bare_block("panic", sequence(vec![
                        literal("WIP".to_owned()),
                    ])),
                )
            },
            StmtNode::Assignment(s) => {
                with_comment(
                    &s.comment,
                    bare_block("assignment", sequence(vec![
                        literal("WIP".to_owned()),
                    ])),
                )
            },
            StmtNode::Line(s) => {
                with_comment(
                    &s.comment,
                    line("", &s.expr),
                )
            },
            StmtNode::EmptyLine() => {
                empty()
            },
        }
    }

    let mut lines = Vec::new();
    write_stmt(stmt)(&mut lines, 0);
    lines.join("\n")
}

fn prettyprint_expr(expr: &ExprNode) -> String {
    fn print_exprs(exprs: &[ExprNode]) -> String {
        exprs.iter().map(print_expr).join(" ")
    }

    fn print_expr(expr: &ExprNode) -> String {
        match expr {
            ExprNode::FuncCall(e) => {
                if e.params.is_empty() {
                    format!(
                        "(func {})", 
                        print_expr(&e.func),
                    )
                } else {
                    format!(
                        "(func {} {})", 
                        print_expr(&e.func),
                        print_exprs(&e.params),
                    )
                }
            },
            ExprNode::ExplicitParenthesis(e) => format!(
                "(paren {})",
                print_expr(e),
            ),
            ExprNode::Infix(e) => {
                format!(
                    "(infix {})",
                    e.exprs.iter()
                        .map(print_expr)
                        .interleave(e.ops.iter().map(InfixOp::to_symbol))
                        .join(" "),
                )
            },
            ExprNode::Prefix(e) => format!(
                "({} {})",
                &e.op.to_symbol(),
                print_expr(&e.expr),
            ),
            ExprNode::Index(e) => format!(
                "(index {} {})",
                print_expr(&e.source),
                print_expr(&e.index),
            ),
            ExprNode::Range(e) => format!(
                "(range {} {})",
                print_expr(&e.start),
                print_expr(&e.end),
            ),
            ExprNode::Lookup(e) => format!(
                "(lookup {} {})",
                print_expr(&e.source),
                e.name_chain.iter().join(" "),
            ),
            ExprNode::Variable(e) => e.to_string(),
            ExprNode::Array(e) => format!(
                "(array {})",
                print_exprs(&e.items),
            ),
            ExprNode::Tuple(e) => format!(
                "(tuple {})",
                print_exprs(&e.items),
            ),
            ExprNode::StringLiteral(e) => quote(e.as_ref()),
            ExprNode::IntLiteral(e) => e.to_string(),
            ExprNode::FloatLiteral(e) => e.to_string(),
            ExprNode::BoolLiteral(e) => e.to_string(),
            ExprNode::Error(e) => format!(
                "(error {})",
                quote(&e.message),
            ),
        }
    }

    print_expr(expr)
}

fn quote(s: impl AsRef<str>) -> String {
    format!("\"{}\"", s.as_ref())
}
