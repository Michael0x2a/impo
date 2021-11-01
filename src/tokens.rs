use std::fmt;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
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
    IntLiteral{
        base: usize,
        digits: String
    },
    FloatLiteral{
        integral_digits: String,
        fractional_digits: String,
        power: String,
    },
    Identifier(String),

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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Position {
    pub lineno: usize,

    // column and offset are w.r.t. characters, not bytes
    pub column: usize,
    pub offset: usize,
}

impl Position {
    pub fn start() -> Position {
        Position{lineno: 0, column: 0, offset: 0}
    }

    pub fn new(lineno: usize, column: usize, offset: usize) -> Position {
        Position{lineno: lineno, column: column, offset: offset}
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

    pub fn add_horizontal(&self, col_offset: usize) -> Position {
        Position { 
            lineno: self.lineno,
            column: self.column + col_offset,
            offset: self.offset + col_offset,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {}, col {}", self.lineno + 1, self.column + 1)
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub position: Position,
}

impl AsRef<Token> for Token {
    fn as_ref(&self) -> &Token {
        self
    }
}