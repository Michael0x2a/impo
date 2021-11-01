use crate::tokens::Position;
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
    StringLiteral(Box<StringLiteralExpr>),
    IntLiteral(Box<IntLiteralExpr>),
    FloatLiteral(Box<FloatLiteralExpr>),
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
    func: ExprNode,
    params: Vec<ExprNode>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct InfixExpr {
    // Invariant: exprs.len() == ops.len() + 1
    exprs: Vec<ExprNode>,
    ops: Vec<Operation>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct IndexExpr {
    source: ExprNode,
    index: ExprNode,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct LookupExpr {
    source: ExprNode,
    field_name: String,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct SliceExpr {
    start: Option<ExprNode>,
    end: Option<ExprNode>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct ArrayExpr {
    items: Vec<ExprNode>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct TupleExpr {
    items: Vec<ExprNode>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct StringLiteralExpr(String);

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct IntLiteralExpr{
    base: usize,
    digits: String
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct FloatLiteralExpr{
    integral_digits: String,
    fractional_digits: String,
    power: String,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct ErrorExpr{
    message: String,
    span: (Position, Position),
}