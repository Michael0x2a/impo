use std::fmt;
use string_cache::DefaultAtom as Atom;
use crate::string_utils::StringJoinExt;

// Represents the concrete name of some defined entity.
// Every item with an identifier must also have a corresponding type.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Identifier{
    pub parent: Vec<Name>,
    pub name: Name,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.parent.iter().map(Name::to_string).join("."), self.name)
    }
}

// Represents the name of some symbol we may or may not be able to look
// up in the current scope.
pub type Name = Atom;
