use std::fmt;
use crate::values::*;
use string_cache::DefaultAtom as Atom;
use strum_macros::IntoStaticStr;

#[derive(Clone, PartialEq, Eq, Debug, Hash, IntoStaticStr)]
pub enum TokenKind {
    // Parentheses
    LParen,
    RParen,
    LBrace,
    RBrace,
    LSquare,
    RSquare,

    // Simple operators (or infix or special meta-symbols...)
    Plus,
    Minus,
    Multiply,
    Divide,
    Percent,
    Equals,
    NotEquals,
    LessThanEquals,
    GreaterThanEquals,
    LessThan,
    GreaterThan,
    Bang,
    Pipe,
    Ampersand,
    Dot,
    Assign,
    Colon,
    Arrow,
    Comma,

    // Literals and identifiers
    StringLiteral(String),
    IntLiteral(IntLiteral),
    FloatLiteral(FloatLiteral),
    BoolLiteral(bool),
    Atom(Atom),

    // Operators (that can also be confused with identifiers)
    InstanceOf,
    Or,
    And,

    // Keywords
    If,
    Elif,
    Else,
    For,
    From,
    To,
    Foreach,
    In,
    While,
    Return,
    Panic,
    Fn,
    Constructor,
    Interface,
    Class,
    Sentinal,
    Const,
    Implements,

    // Misc
    Comment(String),
    Indent,
    Unindent,
    Newline,
    EndOfFile,
}

impl AsRef<TokenKind> for TokenKind {
    fn as_ref(&self) -> &TokenKind {
        self
    }
}

impl TokenKind {
    #[must_use]
    pub fn name(&self) -> &str {
        self.into()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Position {
    pub lineno: usize,

    // column and offset are w.r.t. characters, not bytes
    pub column: usize,
    pub offset: usize,
}

impl Position {
    #[must_use]
    pub fn start() -> Position {
        Position{lineno: 0, column: 0, offset: 0}
    }

    #[must_use]
    pub fn new(lineno: usize, column: usize, offset: usize) -> Position {
        Position{lineno, column, offset}
    }

    pub fn advance(&mut self, c: char) {
        self.offset += 1;
        if c == '\n' {
            self.lineno += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }

    #[must_use]
    pub fn add_horizontal(&self, col_offset: usize) -> Position {
        Position { 
            lineno: self.lineno,
            column: self.column + col_offset,
            offset: self.offset + col_offset,
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.offset.cmp(&other.offset)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.lineno + 1, self.column + 1)
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub position: Position,
}

impl Token {
    #[must_use]
    pub fn span(&self) -> (Position, Position) {
        (self.position, self.end_position())
    }

    #[must_use]
    pub fn end_position(&self) -> Position {
        #[allow(clippy::match_same_arms)]
        let offset = match &self.kind {
            TokenKind::LParen => 1,
            TokenKind::RParen => 1,
            TokenKind::LBrace => 1,
            TokenKind::RBrace => 1,
            TokenKind::LSquare => 1,
            TokenKind::RSquare => 1,
            TokenKind::Plus => 1,
            TokenKind::Minus => 1,
            TokenKind::Multiply => 1,
            TokenKind::Divide => 1,
            TokenKind::Percent => 1,
            TokenKind::Equals => 2,
            TokenKind::NotEquals => 2,
            TokenKind::LessThanEquals => 2,
            TokenKind::GreaterThanEquals => 2,
            TokenKind::LessThan => 1,
            TokenKind::GreaterThan => 1,
            TokenKind::Bang => 1,
            TokenKind::Pipe => 1,
            TokenKind::Ampersand => 1,
            TokenKind::Dot => 1,
            TokenKind::Assign => 1,
            TokenKind::Colon => 1,
            TokenKind::Arrow => 2,
            TokenKind::Comma => 1,
            TokenKind::StringLiteral(s) => s.chars().count(),
            TokenKind::IntLiteral(lit) => lit.char_len(),
            TokenKind::FloatLiteral(lit) => lit.char_len(),
            TokenKind::BoolLiteral(lit) => if *lit { 4 } else { 5 },
            TokenKind::Atom(atom) => atom.chars().count(),
            TokenKind::Comment(text) => 2 + text.chars().count(),
            TokenKind::Indent => 0,
            TokenKind::Unindent => 0,
            TokenKind::Newline => 0,
            TokenKind::EndOfFile => 0,
            rest => rest.name().chars().count(),
        };
        self.position.add_horizontal(offset)
    }
}

impl AsRef<Token> for Token {
    fn as_ref(&self) -> &Token {
        self
    }
}