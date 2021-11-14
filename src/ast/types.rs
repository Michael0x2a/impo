use super::primitives::Name;
use crate::tokens::Position;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum TypeNode {
    Class(Box<ClassType>),
    Interface(Box<InterfaceType>),
    Sentinal(Box<SentinalType>),
    Func(Box<FuncType>),
    Union(Box<UnionType>),
    Tuple(Box<TupleType>),
    TypeVar(Box<TypeVarType>),
    Error(Box<ErrorType>),
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct ClassType {
    name: Name,
    typevars: Vec<TypeVarType>,
    entries: Vec<(Name, TypeNode)>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct InterfaceType {
    name: Name,
    typevars: Vec<TypeVarType>,
    entries: Vec<(Name, TypeNode)>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct SentinalType {
    name: Name,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct FuncType {
    parent: Option<Box<InterfaceType>>,
    typevars: Vec<TypeVarType>,
    parameters: Vec<TypeNode>,
    return_type: TypeNode,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct UnionType {
    variants: Vec<TypeNode>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct TupleType {
    items: Vec<TypeNode>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct TypeVarType {
    name: Name,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct ErrorType {
    message: String,
    span: (Position, Position),
}