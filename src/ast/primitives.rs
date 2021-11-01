
// Represents the concrete name of some defined entity.
// Every item with an identifier must also have a corresponding type.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Identifier{
    parent: Vec<Name>,
    name: Name,
}

// Represents the name of some symbol we may or may not be able to look
// up in the current scope.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Name(String);
