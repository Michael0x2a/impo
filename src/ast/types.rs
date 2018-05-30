
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum ImpoType {
    Nil,
    Bottom,
    Any(AnyKind),
    Primitive(PrimitiveKind),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AnyKind {
    FromError,
    Deliberate,
    Inferred,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum PrimitiveKind {
    Bool,
    Int,
    Float,
    String,
}
