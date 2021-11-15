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
pub struct InfixExpr {
    // Invariants:
    // - exprs.len() == ops.len() + 1
    // - All operations in the vec have the same precedence and
    //   can be evaluated from left to right
    pub exprs: Vec<ExprNode>,
    pub ops: Vec<Operator>,
}

impl From<InfixExpr> for ExprNode {
    fn from(other: InfixExpr) -> ExprNode {
        ExprNode::Infix(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Operator {
    Multiplication,
    Division,
    Addition,
    Subtraction,
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

impl Operator {
    // Returns how "strongly" this operator binds itself to
    // the expression on the left and right. Higher numbers results
    // in a "stronger" binding.
    #[must_use]
    pub fn binding_power(&self) -> (u8, u8) {
        match &self {
            Operator::LogicalOr => (1, 2),
            Operator::LogicalAnd => (3, 4),
            Operator::InstanceOf => (5, 6),

            // Infix: logical negate here?

            // Comparisons
            Operator::Equals 
            | Operator::NotEquals
            | Operator::LessThanEquals
            | Operator::GreaterThanEquals
            | Operator::LessThan
            | Operator::GreaterThan => (7, 8),

            // Bitwise operations
            Operator::BitwiseOr => (9, 10),
            Operator::BitwiseAnd => (11, 12),

            // Arithmetic
            Operator::Addition
            | Operator::Subtraction => (13, 14),

            Operator::Multiplication
            | Operator::Division
            | Operator::Modulus => (15, 16),
        }
    }

    #[must_use]
    pub fn to_symbol(&self) -> String {
        match &self {
            Operator::Multiplication => "*",
            Operator::Division => "/",
            Operator::Addition => "+",
            Operator::Subtraction => "-",
            Operator::Modulus => "%",
            Operator::Equals => "==",
            Operator::NotEquals => "!=",
            Operator::LessThanEquals => "<=",
            Operator::GreaterThanEquals => ">=",
            Operator::LessThan => "<",
            Operator::GreaterThan => ">",
            Operator::BitwiseOr => "|",
            Operator::BitwiseAnd => "&",
            Operator::LogicalOr => "or",
            Operator::LogicalAnd => "and",
            Operator::InstanceOf => "instanceof",
        }.to_owned()
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct FuncCallExpr {
    pub func: ExprNode,
    pub params: Vec<ExprNode>,
}

impl From<FuncCallExpr> for ExprNode {
    fn from(other: FuncCallExpr) -> ExprNode {
        ExprNode::FuncCall(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct IndexExpr {
    pub source: ExprNode,
    pub index: ExprNode,
}

impl From<IndexExpr> for ExprNode {
    fn from(other: IndexExpr) -> ExprNode {
        ExprNode::Index(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct LookupExpr {
    pub source: ExprNode,
    pub name_chain: Vec<Name>,
}

impl From<LookupExpr> for ExprNode {
    fn from(other: LookupExpr) -> ExprNode {
        ExprNode::Lookup(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct SliceExpr {
    pub start: Option<ExprNode>,
    pub end: Option<ExprNode>,
}

impl From<SliceExpr> for ExprNode {
    fn from(other: SliceExpr) -> ExprNode {
        ExprNode::Slice(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct ArrayExpr {
    pub items: Vec<ExprNode>,
}

impl From<ArrayExpr> for ExprNode {
    fn from(other: ArrayExpr) -> ExprNode {
        ExprNode::Array(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct TupleExpr {
    pub items: Vec<ExprNode>,
}

impl From<TupleExpr> for ExprNode {
    fn from(other: TupleExpr) -> ExprNode {
        ExprNode::Tuple(Box::new(other))
    }
}


#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct ErrorExpr{
    pub message: String,
    pub span: (Position, Position),
}

impl<S: AsRef<str>> From<S> for ExprNode {
    fn from(other: S) -> ExprNode {
        ExprNode::StringLiteral(Box::new(other.as_ref().to_owned()))
    }
}

impl From<IntLiteral> for ExprNode {
    fn from(other: IntLiteral) -> ExprNode {
        ExprNode::IntLiteral(Box::new(other))
    }
}

impl From<FloatLiteral> for ExprNode {
    fn from(other: FloatLiteral) -> ExprNode {
        ExprNode::FloatLiteral(Box::new(other))
    }
}
