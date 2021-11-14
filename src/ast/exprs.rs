use crate::tokens::Position;
use crate::values::*;
use super::primitives::Name;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum ExprNode{
    FuncCall(Box<FuncCallExpr>),
    ExplicitParenthesis(Box<ExprNode>),
    Infix(Box<InfixExpr>),
    LogicalNegate(Box<ExprNode>),
    NumericalNegate(Box<ExprNode>),
    Index(Box<IndexExpr>),
    Slice(Box<SliceExpr>),
    Lookup(Box<LookupExpr>),
    Variable(Name),
    Array(Box<ArrayExpr>),
    Tuple(Box<TupleExpr>),
    StringLiteral(Box<String>),
    IntLiteral(Box<IntLiteral>),
    FloatLiteral(Box<FloatLiteral>),
    BoolLiteral(bool),
    Error(Box<ErrorExpr>),
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulus,

    Equals,
    NotEquals,
    LessThanEquals,
    GreaterThanEquals,
    LessThan,
    GreaterThan,

    BitwiseOr,
    BitwiseAnd,

    LogicalOr,
    LogicalAnd,

    InstanceOf,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct FuncCallExpr {
    pub func: ExprNode,
    pub params: Vec<ExprNode>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct InfixExpr {
    // Invariant: exprs.len() == ops.len() + 1
    pub exprs: Vec<ExprNode>,
    pub ops: Vec<Operation>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct IndexExpr {
    pub source: ExprNode,
    pub index: ExprNode,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct LookupExpr {
    pub source: ExprNode,
    pub name_chain: Vec<Name>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct SliceExpr {
    pub start: Option<ExprNode>,
    pub end: Option<ExprNode>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct ArrayExpr {
    pub items: Vec<ExprNode>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct TupleExpr {
    pub items: Vec<ExprNode>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct StringLiteralExpr(String);

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct ErrorExpr{
    pub message: String,
    pub span: (Position, Position),
}