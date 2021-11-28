use super::exprs::ExprNode;
use super::primitives::{Identifier, Name};

type Block = Vec<StmtNode>;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum StmtNode{
    Program(Box<Program>),

    // Module declarations
    Import(Box<ImportStmt>),

    // Definitions
    InterfaceDef(Box<InterfaceDefStmt>),
    ClassDef(Box<ClassDefStmt>),
    SentinalDef(Box<SentinalDefStmt>),
    FieldSignatureDef(Box<FieldSignatureDefStmt>),
    FuncSignatureDef(Box<FuncSignatureDefStmt>),
    FuncImplementationDef(Box<FuncImplementationDefStmt>),

    // Control flow
    If(Box<IfStmt>),
    For(Box<ForStmt>),
    Foreach(Box<ForeachStmt>),
    While(Box<WhileStmt>),
    Return(Box<ReturnStmt>),
    Panic(Box<PanicStmt>),

    // Line-based primitives
    Assignment(Box<AssignmentStmt>),
    Line(Box<LineStmt>),
    EmptyLine(),
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Program {
    pub body: Block
}

impl From<Program> for StmtNode {
    fn from(other: Program) -> StmtNode {
        StmtNode::Program(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Comment {
    pub lines: Vec<String>,
}

impl Comment {
    #[must_use]
    pub fn new(lines: Vec<String>) -> Comment {
        Comment{lines: lines}
    }

    #[must_use]
    pub fn empty() -> Comment {
        Comment{lines: Vec::new()}
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct ImportStmt {
    pub comment: Comment,
    pub source: Identifier,
    pub imports: Vec<Name>,
}

impl From<ImportStmt> for StmtNode {
    fn from(other: ImportStmt) -> StmtNode {
        StmtNode::Import(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct InterfaceDefStmt {
    pub comment: Comment,
    pub identifier: Identifier,
    pub fields: Vec<FieldSignatureDefStmt>,
    pub functions: Vec<FuncSignatureDefStmt>,
}

impl From<InterfaceDefStmt> for StmtNode {
    fn from(other: InterfaceDefStmt) -> StmtNode {
        StmtNode::InterfaceDef(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct ClassDefStmt {
    pub comment: Comment,
    pub identifier: Identifier,
    pub implements: Option<Identifier>,
    pub fields: Vec<FieldSignatureDefStmt>,
    pub functions: Vec<FuncImplementationDefStmt>,
}

impl From<ClassDefStmt> for StmtNode {
    fn from(other: ClassDefStmt) -> StmtNode {
        StmtNode::ClassDef(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct SentinalDefStmt {
    pub comment: Comment,
    pub identifier: Identifier,
}

impl From<SentinalDefStmt> for StmtNode {
    fn from(other: SentinalDefStmt) -> StmtNode {
        StmtNode::SentinalDef(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct FieldSignatureDefStmt {
    pub comment: Comment,
    pub identifier: Identifier,
}

impl From<FieldSignatureDefStmt> for StmtNode {
    fn from(other: FieldSignatureDefStmt) -> StmtNode {
        StmtNode::FieldSignatureDef(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct FuncSignatureDefStmt {
    pub comment: Comment,
    pub identifier: Identifier,
    pub params: Vec<Name>,
}

impl From<FuncSignatureDefStmt> for StmtNode {
    fn from(other: FuncSignatureDefStmt) -> StmtNode {
        StmtNode::FuncSignatureDef(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct FuncImplementationDefStmt {
    pub comment: Comment,
    pub identifier: Identifier,
    pub params: Vec<Name>,
    pub body: Block,
}

impl From<FuncImplementationDefStmt> for StmtNode {
    fn from(other: FuncImplementationDefStmt) -> StmtNode {
        StmtNode::FuncImplementationDef(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct IfStmt {
    pub comment: Comment,
    pub branches: Vec<(ExprNode, Block)>,
    pub fallback_branch: Block,
}

impl From<IfStmt> for StmtNode {
    fn from(other: IfStmt) -> StmtNode {
        StmtNode::If(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct ForStmt{
    pub comment: Comment,
    pub variable: Name,
    pub start: i64,
    pub end: i64,
    pub body: Block,
}

impl From<ForStmt> for StmtNode {
    fn from(other: ForStmt) -> StmtNode {
        StmtNode::For(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct ForeachStmt {
    pub comment: Comment,
    pub variables: Vec<Name>,
    pub iterable: ExprNode,
    pub body: Block,
}

impl From<ForeachStmt> for StmtNode {
    fn from(other: ForeachStmt) -> StmtNode {
        StmtNode::Foreach(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct WhileStmt {
    pub comment: Comment,
    pub cond: ExprNode,
    pub body: Block,
}

impl From<WhileStmt> for StmtNode {
    fn from(other: WhileStmt) -> StmtNode {
        StmtNode::While(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct ReturnStmt {
    pub comment: Comment,
    pub value: Option<ExprNode>,
}

impl From<ReturnStmt> for StmtNode {
    fn from(other: ReturnStmt) -> StmtNode {
        StmtNode::Return(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct PanicStmt {
    pub comment: Comment,
    pub value: Option<ExprNode>,
}

impl From<PanicStmt> for StmtNode {
    fn from(other: PanicStmt) -> StmtNode {
        StmtNode::Panic(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct AssignmentStmt {
    pub comment: Comment,
    pub targets: Vec<ExprNode>,
    pub value: ExprNode,
}

impl From<AssignmentStmt> for StmtNode {
    fn from(other: AssignmentStmt) -> StmtNode {
        StmtNode::Assignment(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct LineStmt {
    pub comment: Comment,
    pub expr: ExprNode,
}

impl From<LineStmt> for StmtNode {
    fn from(other: LineStmt) -> StmtNode {
        StmtNode::Line(Box::new(other))
    }
}