use std::fmt;

use string_cache::DefaultAtom as Atom;
use derive_more::Constructor;
use struple::Struple;

use crate::string_utils::StringJoinExt;

// Represents the full name of some defined entity. This identifier can
// be used to look up the entity name in any scope.
// Every item with an identifier must also have a corresponding type.
pub struct AbsoluteIdentifier(Identifier);

// Represents the name of some defined identity, relative to a specific
// scope. You need this scope/this context to fully resolve the location
// of this identifier.
pub struct RelativeIdentifier(Identifier);


#[derive(Clone, PartialEq, Eq, Debug, Hash, Constructor, Struple)]
pub struct Identifier {
    pub parent: Vec<Name>,
    pub name: Name,
}

impl Identifier {
    fn new_from_string(s: impl AsRef<str>) -> Identifier {
        let parts = s.as_ref().split('.').collect::<Vec<_>>();
        Identifier{
            parent: parts.get(0..parts.len()-1)
                .into_iter()
                .flat_map(|p| p.iter())
                .map(|&p| p.into())
                .collect(),
            name: parts
                .last()
                .copied()
                .unwrap_or("<MALFORMED_IDENTIFIER>")
                .into(),
        }
    }
}

impl From<&str> for Identifier {
    fn from(s: &str) -> Self {
        Identifier::new_from_string(s)
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.parent.is_empty() {
            write!(f, "{}", self.name)
        } else {
            write!(f, "{}.{}", self.parent.iter().map(Name::to_string).join("."), self.name)
        }
    }
}

// Represents the name of some symbol we may or may not be able to look
// up in the current scope.
pub type Name = Atom;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_identifier_creation() {
        assert_eq!(
            Identifier::new_from_string("foo"),
            Identifier{
                parent: Vec::new(),
                name: "foo".into(),
            },
        );

        assert_eq!(
            Identifier::new_from_string("foo.bar.baz"),
            Identifier{
                parent: vec!["foo".into(), "bar".into()],
                name: "baz".into(),
            },
        );
    }
}