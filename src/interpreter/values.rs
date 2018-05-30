use ast::stringpool::StringPoolId;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Float(u64),
    String(StringPoolId),
    Nil,
}