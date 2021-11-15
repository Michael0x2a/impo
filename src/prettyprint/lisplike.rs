use itertools::Itertools;
use crate::ast::*;
use crate::string_utils::StringJoinExt;

#[must_use]
pub fn prettyprint_program(stmts: &[StmtNode]) -> String {
    stmts.iter().map(prettyprint_stmt).join("")
}

#[allow(clippy::too_many_lines)]
fn prettyprint_stmt(stmt: &StmtNode) -> String {
    fn line(level: usize, text: impl Into<String>) -> String {
        format!("{}{}", " ".repeat(level), text.into())
    }

    fn write_comment(lines: &mut Vec<String>, level: usize, comment: &Comment) {
        for comment in &comment.lines {
            lines.push(line(level, format!("# {}", comment)));
        }
    }

    fn block_writer(
        lines: &mut Vec<String>,
        level: usize,
        name: &'static str,
        comment: &Comment,
        children: Vec<String>,
    ) {
        write_comment(lines, level, comment);
        lines.push(line(level, format!("({}", name)));
        lines.extend(children);
        lines.push(line(level, ")"));
    }

    fn write_stmt(lines: &mut Vec<String>, curr: usize, stmt: &StmtNode) {
        let mut block = |name, comment, children| {
            block_writer(lines, curr, name, comment, children);
        };

        let next = curr + 4;
        match stmt {
            StmtNode::Import(s) => {
                block("import", &s.comment, vec![
                    line(next, s.source.to_string()),
                    line(next, s.imports.iter().map(quote).join(" ")),
                ]);
            },
            StmtNode::InterfaceDef(s) => {
                block("interface", &s.comment, vec![
                    line(next, "WIP"),
                ]);
            },
            StmtNode::ClassDef(s) => {
                block("class", &s.comment, vec![
                    line(next, "WIP"),
                ]);
            },
            StmtNode::SentinalDef(s) => {
                block("sentinal", &s.comment, vec![
                    line(next, "WIP"),
                ]);
            },
            StmtNode::FieldSignatureDef(s) => {
                block("field", &s.comment, vec![
                    line(next, "WIP"),
                ]);
            },
            StmtNode::FuncSignatureDef(s) => {
                block("fn", &s.comment, vec![
                    line(next, "WIP"),
                ]);
            },
            StmtNode::FuncImplementationDef(s) => {
                block("fn", &s.comment, vec![
                    line(next, "WIP"),
                ]);
            },
            StmtNode::If(s) => {
                block("if", &s.comment, vec![
                    line(next, "WIP"),
                ]);
            },
            StmtNode::For(s) => {
                block("for", &s.comment, vec![
                    line(next, "WIP"),
                ]);
            },
            StmtNode::Foreach(s) => {
                block("foreach", &s.comment, vec![
                    line(next, "WIP"),
                ]);
            },
            StmtNode::While(s) => {
                block("while", &s.comment, vec![
                    line(next, "WIP"),
                ]);
            },
            StmtNode::Return(s) => {
                block("return", &s.comment, vec![
                    line(next, "WIP"),
                ]);
            },
            StmtNode::Panic(s) => {
                block("panic", &s.comment, vec![
                    line(next, "WIP"),
                ]);
            },
            StmtNode::Assignment(s) => {
                block("assignment", &s.comment, vec![
                    line(next, "WIP"),
                ]);
            },
            StmtNode::Line(s) => {
                write_comment(lines, curr, &s.comment);
                lines.push(line(curr, prettyprint_expr(&s.expr)));
            },
        }
    }

    let mut lines = Vec::new();
    write_stmt(&mut lines, 0, stmt);
    lines.join("")
}

fn prettyprint_expr(expr: &ExprNode) -> String {
    fn print_exprs(exprs: &[ExprNode]) -> String {
        exprs.iter().map(print_expr).join(" ")
    }

    fn print_expr(expr: &ExprNode) -> String {
        match expr {
            ExprNode::FuncCall(e) => format!(
                "(func {} {})", 
                print_expr(&e.func),
                print_exprs(&e.params),
            ),
            ExprNode::ExplicitParenthesis(e) => format!(
                "(paren {})",
                print_expr(e),
            ),
            ExprNode::Infix(e) => {
                format!(
                    "(infix {})",
                    e.exprs.iter()
                        .map(print_expr)
                        .interleave(e.ops.iter().map(Operator::to_symbol))
                        .join(" "),
                )
            },
            ExprNode::LogicalNegate(e) => format!(
                "(! {})",
                print_expr(e),
            ),
            ExprNode::NumericalNegate(e) => format!(
                "(- {})",
                print_expr(e),
            ),
            ExprNode::Index(e) => format!(
                "(index {} {})",
                print_expr(&e.source),
                print_expr(&e.index),
            ),
            ExprNode::Slice(e) => format!(
                "(slice {} {})",
                &e.start.as_ref().map_or_else(|| "start".to_owned(), print_expr),
                &e.start.as_ref().map_or_else(|| "end".to_owned(), print_expr),
            ),
            ExprNode::Lookup(e) => format!(
                "(lookup {} {})",
                print_expr(&e.source),
                e.name_chain.iter().map(quote).join(" "),
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
