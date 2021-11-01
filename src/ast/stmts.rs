use super::exprs::ExprNode;
use super::primitives::{Identifier, Name};

type Block = Vec<StmtNode>;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum StmtNode{
    // Module declarations
    Import(Box<ImportStmt>),

    // Definitions
    InterfaceDef(Box<InterfaceDefStmt>),
    ClassDef(Box<ClassDefStmt>),
    SentinalDef(Box<SentinalDefStmt>),
    FuncImplementationDef(Box<FuncImplementationDefStmt>),

    // Control flow
    If(Box<IfStmt>),
    For(Box<ForStatement>),
    Foreach(Box<ForeachStmt>),
    While(Box<WhileStmt>),
    Return(Box<ReturnStmt>),
    Panic(Box<PanicStmt>),

    // Line-based primitives
    Assignment(Box<AssignmentStmt>),
    Line(ExprNode),
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Comment {
    lines: Vec<String>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct ImportStmt {
    source: Identifier,
    imports: Vec<Name>,
}


#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct InterfaceDefStmt {
    comment: Comment,
    identifier: Identifier,
    fields: Vec<FieldSignatureDefStmt>,
    functions: Vec<FuncSignatureDefStmt>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct ClassDefStmt {
    comment: Comment,
    identifier: Identifier,
    implements: Option<Identifier>,
    fields: Vec<FieldSignatureDefStmt>,
    functions: Vec<FuncImplementationDefStmt>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct SentinalDefStmt {
    comment: Comment,
    identifier: Identifier,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct FieldSignatureDefStmt {
    comment: Comment,
    identifier: Identifier,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct FuncSignatureDefStmt {
    comment: Comment,
    identifier: Identifier,
    params: Vec<String>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct FuncImplementationDefStmt {
    comment: Comment,
    identifier: Identifier,
    params: Vec<String>,
    body: Block,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct IfStmt {
    comment: Comment,
    branches: Vec<(ExprNode, Block)>,
    fallback_branch: Block,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct ForStatement{
    comment: Comment,
    variable: String,
    start: i64,
    end: i64,
    body: Block,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct ForeachStmt {
    comment: Comment,
    variables: Vec<String>,
    iterable: ExprNode,
    body: Block,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct WhileStmt {
    comment: Comment,
    cond: ExprNode,
    body: Block,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct ReturnStmt {
    comment: Comment,
    value: Option<ExprNode>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct PanicStmt {
    comment: Comment,
    value: Option<ExprNode>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct AssignmentStmt {
    comment: Comment,
    targets: Vec<ExprNode>,
    value: ExprNode,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct LineStmt {
    comment: Comment,
    expr: ExprNode,
}