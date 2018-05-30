use parser::tokens::TokenType;
use snowflake::ProcessUniqueId;
use ast::stringpool::StringPoolId;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NodeId(ProcessUniqueId);

impl NodeId {
    pub fn new() -> NodeId {
        NodeId(ProcessUniqueId::new())
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum ExprNode<EType> {
    BinaryOp { kind: BinaryOpKind, left: Box<EType>, right: Box<EType> },
    UnaryOp { kind: UnaryOpKind, item: Box<EType> },
    Group(Box<EType>),
    BooleanLiteral(bool),
    IntLiteral(i64),
    // Note: We store the float as u64 because f64 doesn't implement
    // Eq or Hash.
    // Use my_float.to_bits() or f64.from_bits(my_unsigned) to
    // convert between the two.
    FloatLiteral(u64),
    StringLiteral(StringPoolId),
    NilLiteral,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum BinaryOpKind {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    FloorDivision,
    EqualTo,
    NotEqualTo,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Mod,
    Exponentiate
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum UnaryOpKind {
    BooleanNegation,
    NumericNegation,
}

pub fn map_token_type_to_binop_kind(token_type: TokenType) -> BinaryOpKind {
    match token_type {
        TokenType::Plus => BinaryOpKind::Addition,
        TokenType::Minus => BinaryOpKind::Subtraction,
        TokenType::Star => BinaryOpKind::Multiplication,
        TokenType::Slash => BinaryOpKind::Division,
        TokenType::SlashSlash => BinaryOpKind::FloorDivision,
        TokenType::Percent => BinaryOpKind::Mod,
        TokenType::Caret => BinaryOpKind::Exponentiate,
        TokenType::EqualEqual => BinaryOpKind::EqualTo,
        TokenType::BangEqual => BinaryOpKind::NotEqualTo,
        TokenType::Greater => BinaryOpKind::GreaterThan,
        TokenType::GreaterEqual => BinaryOpKind::GreaterThanOrEqual,
        TokenType::Less => BinaryOpKind::LessThan,
        TokenType::LessEqual => BinaryOpKind::LessThanOrEqual,
        _ => panic!("Unexpected token type {:?}", token_type),
    }
}

pub fn map_binop_kind_to_str(kind: BinaryOpKind) -> &'static str {
    match kind {
        BinaryOpKind::Addition => "+",
        BinaryOpKind::Subtraction => "-",
        BinaryOpKind::Multiplication => "*",
        BinaryOpKind::Division => "/",
        BinaryOpKind::FloorDivision => "//",
        BinaryOpKind::Mod => "%",
        BinaryOpKind::Exponentiate => "^",
        BinaryOpKind::EqualTo => "==",
        BinaryOpKind::NotEqualTo => "!=",
        BinaryOpKind::GreaterThan => ">",
        BinaryOpKind::GreaterThanOrEqual => ">=",
        BinaryOpKind::LessThan => "<",
        BinaryOpKind::LessThanOrEqual => "<=",
    }
}

pub fn map_token_type_to_unary_kind(token_type: TokenType) -> UnaryOpKind {
    match token_type {
        TokenType::Minus => UnaryOpKind::NumericNegation,
        TokenType::Bang => UnaryOpKind::BooleanNegation,
        _ => panic!("Unexpected token type {:?}", token_type),
    }
}

pub fn map_unary_kind_to_str(kind: UnaryOpKind) -> &'static str {
    match kind {
        UnaryOpKind::BooleanNegation => "!",
        UnaryOpKind::NumericNegation => "-",
    }
}
