use struple::Struple;
use crate::tokens::Position;
use crate::values::*;
use super::primitives::Name;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum ExprNode{
    FuncCall(Box<FuncCallExpr>),
    ExplicitParenthesis(Box<ExprNode>),
    Infix(Box<InfixExpr>),
    Prefix(Box<PrefixExpr>),
    Index(Box<IndexExpr>),
    Range(Box<RangeExpr>),
    FieldLookup(Box<FieldLookupExpr>),
    TupleLookup(Box<TupleLookupExpr>),
    Variable(Name),
    Array(Box<ArrayExpr>),
    Tuple(Box<TupleExpr>),
    StringLiteral(Box<String>),
    IntLiteral(Box<IntLiteral>),
    FloatLiteral(Box<FloatLiteral>),
    BoolLiteral(bool),
    Error(Box<ErrorExpr>),
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Struple)]
pub struct InfixExpr {
    // Invariants:
    // - exprs.len() == ops.len() + 1
    // - All operations in the vec have the same precedence and
    //   can be evaluated from left to right
    pub exprs: Vec<ExprNode>,
    pub ops: Vec<InfixOp>,
}

impl From<InfixExpr> for ExprNode {
    fn from(other: InfixExpr) -> ExprNode {
        ExprNode::Infix(Box::new(other))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum InfixOp {
    LogicalOr,
    LogicalAnd,

    Equals,
    NotEquals,
    LessThanEquals,
    GreaterThanEquals,
    LessThan,
    GreaterThan,
    To,

    InstanceOf,

    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,

    BitwiseShiftLeft,
    BitwiseShiftRight,

    Addition,
    Subtraction,

    Multiplication,
    Division,
    Modulus,
}

impl InfixOp {
    // Returns how "strongly" this operator binds itself to
    // the expression on the left and right. Higher numbers results
    // in a "stronger" binding.
    #[must_use]
    pub fn binding_power(&self) -> (u8, u8) {
        match &self {
            InfixOp::LogicalOr => (1, 2),
            InfixOp::LogicalAnd => (3, 4),

            // (5, 6) is reserved by the logical negation prefix operator

            // Comparisons
            InfixOp::Equals 
            | InfixOp::NotEquals
            | InfixOp::LessThanEquals
            | InfixOp::GreaterThanEquals
            | InfixOp::LessThan
            | InfixOp::GreaterThan => (7, 8),
           
            // Range
            InfixOp::To => (9, 10),

            // Special comparisons`
            InfixOp::InstanceOf => (11, 12),

            // Bitwise operations
            InfixOp::BitwiseOr => (13, 14),
            InfixOp::BitwiseXor => (15, 16),
            InfixOp::BitwiseAnd => (17, 18),

            // Bitwise shifts
            InfixOp::BitwiseShiftLeft
            | InfixOp::BitwiseShiftRight => (19, 20),

            // Arithmetic
            InfixOp::Addition
            | InfixOp::Subtraction => (21, 22),

            InfixOp::Multiplication
            | InfixOp::Division
            | InfixOp::Modulus => (23, 24),

            // (25, 26) is reserved by the numerical and bitwise prefix operators
        }
    }

    #[must_use]
    pub fn to_symbol(&self) -> String {
        match &self {
            InfixOp::LogicalOr => "or",
            InfixOp::LogicalAnd => "and",

            InfixOp::Equals => "==",
            InfixOp::NotEquals => "!=",
            InfixOp::LessThanEquals => "<=",
            InfixOp::GreaterThanEquals => ">=",
            InfixOp::LessThan => "<",
            InfixOp::GreaterThan => ">",
            InfixOp::To => "to",

            InfixOp::InstanceOf => "instanceof",

            InfixOp::BitwiseOr => "|",
            InfixOp::BitwiseXor => "^",
            InfixOp::BitwiseAnd => "&",

            InfixOp::BitwiseShiftLeft => "<<",
            InfixOp::BitwiseShiftRight => ">>",

            InfixOp::Addition => "+",
            InfixOp::Subtraction => "-",

            InfixOp::Multiplication => "*",
            InfixOp::Division => "/",
            InfixOp::Modulus => "%",
        }.to_owned()
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Struple)]
pub struct PrefixExpr {
    pub expr: ExprNode,
    pub op: PrefixOp,
}

impl From<PrefixExpr> for ExprNode {
    fn from(other: PrefixExpr) -> ExprNode {
        ExprNode::Prefix(Box::new(other))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum PrefixOp {
    LogicalNegate,
    NumericalNegate,
    BitwiseNegate,
}

impl PrefixOp {
    // Returns how "strongly" this operator binds itself to
    // the expression on the left and right. Higher numbers results
    // in a "stronger" binding.
    //
    // Since prefixes only bind to the right, we mandate we return
    // the unit type for the left.
    #[must_use]
    pub fn binding_power(&self) -> ((), u8) {
        match &self {
            PrefixOp::LogicalNegate => ((), 6),
            PrefixOp::NumericalNegate
            | PrefixOp::BitwiseNegate => ((), 26),
        }
    }

    #[must_use]
    pub fn to_symbol(&self) -> String {
        match &self {
            PrefixOp::LogicalNegate => "!",
            PrefixOp::NumericalNegate => "-",
            PrefixOp::BitwiseNegate => "~",
        }.to_owned()
    }
}


#[derive(Clone, PartialEq, Eq, Debug, Hash, Struple)]
pub struct FuncCallExpr {
    pub func: ExprNode,
    pub params: Vec<ExprNode>,
}

impl From<FuncCallExpr> for ExprNode {
    fn from(other: FuncCallExpr) -> ExprNode {
        ExprNode::FuncCall(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Struple)]
pub struct IndexExpr {
    pub source: ExprNode,
    pub index: ExprNode,
}

impl From<IndexExpr> for ExprNode {
    fn from(other: IndexExpr) -> ExprNode {
        ExprNode::Index(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Struple)]
pub struct FieldLookupExpr {
    pub source: ExprNode,
    pub name_chain: Vec<Name>,
}

impl From<FieldLookupExpr> for ExprNode {
    fn from(other: FieldLookupExpr) -> ExprNode {
        ExprNode::FieldLookup(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Struple)]
pub struct TupleLookupExpr {
    pub source: ExprNode,
    pub index_chain: Vec<usize>,
}

impl From<TupleLookupExpr> for ExprNode {
    fn from(other: TupleLookupExpr) -> ExprNode {
        ExprNode::TupleLookup(Box::new(other))
    }
}


#[derive(Clone, PartialEq, Eq, Debug, Hash, Struple)]
pub struct RangeExpr {
    pub start: ExprNode,
    pub end: ExprNode,
}

impl From<RangeExpr> for ExprNode {
    fn from(other: RangeExpr) -> ExprNode {
        ExprNode::Range(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Struple)]
pub struct ArrayExpr {
    pub items: Vec<ExprNode>,
}

impl From<ArrayExpr> for ExprNode {
    fn from(other: ArrayExpr) -> ExprNode {
        ExprNode::Array(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Struple)]
pub struct TupleExpr {
    pub items: Vec<ExprNode>,
}

impl TupleExpr {
    #[must_use]
    pub fn new(items: Vec<ExprNode>) -> TupleExpr {
        TupleExpr{ items: items }
    }
}

impl From<TupleExpr> for ExprNode {
    fn from(other: TupleExpr) -> ExprNode {
        ExprNode::Tuple(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Struple)]
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
