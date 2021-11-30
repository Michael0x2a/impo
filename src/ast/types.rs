use derive_more::Constructor;
use struple::Struple;

use super::primitives::{Name, Identifier};
use crate::tokens::Position;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum TypeNode {
    Reference(Box<ReferenceType>),
    Func(Box<FuncType>),
    Union(Box<UnionType>),
    Tuple(Box<TupleType>),
    Unit,
    Empty,
    Error(Box<ErrorType>),
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Constructor, Struple)]
pub struct ReferenceType {
    pub identifier: Identifier,
    pub type_params: Vec<TypeNode>,
}

impl From<ReferenceType> for TypeNode {
    fn from(other: ReferenceType) -> TypeNode {
        TypeNode::Reference(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Constructor, Struple)]
pub struct FuncType {
    pub typevars: Vec<Name>,
    pub param_types: Vec<TypeNode>,
    pub return_type: TypeNode,
}

impl From<FuncType> for TypeNode {
    fn from(other: FuncType) -> TypeNode {
        TypeNode::Func(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Constructor, Struple)]
pub struct UnionType {
    pub variants: Vec<TypeNode>,
}

impl From<UnionType> for TypeNode {
    fn from(other: UnionType) -> TypeNode {
        TypeNode::Union(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Constructor, Struple)]
pub struct TupleType {
    pub items: Vec<TypeNode>,
}

impl From<TupleType> for TypeNode {
    fn from(other: TupleType) -> TypeNode {
        TypeNode::Tuple(Box::new(other))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Constructor, Struple)]
pub struct ErrorType {
    pub message: String,
    pub span: (Position, Position),
}

impl From<ErrorType> for TypeNode {
    fn from(other: ErrorType) -> TypeNode {
        TypeNode::Error(Box::new(other))
    }
}