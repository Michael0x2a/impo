use text::{TextOffset, TextRange};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Percent,
    Semicolon,
    Colon,
    Slash,
    Star,
    Caret,
    QuestionMark,
    Hash,
    Newline,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Arrow,
    SlashSlash,

    // Literals
    Identifier,
    StringLiteral,
    IntLiteral,
    FloatLiteral,

    // Keywords
    And,
    Or,
    Class,
    If,
    Else,
    True,
    False,
    Fn,
    While,
    Repeat,
    Nil,
    Print,
    Return,
    Super,
    This,
    Var,
    Dyn,

    // Other
    Eof,
    UnexpectedSequence,
    Placeholder,
    Comment,
    Whitespace,
}

pub static PLACEHOLDER_TOKEN: Token =
    Token {
        token_type: TokenType::Placeholder,
        location: TextRange { offset: TextOffset { offset: 0 }, length: 0 },
        lexeme: None,
    };

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub location: TextRange,
    pub lexeme: Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType, location: TextRange, lexeme: String) -> Token {
        Token { token_type, location, lexeme: Some(lexeme) }
    }

    pub fn new_empty(token_type: TokenType, location: TextRange) -> Token {
        Token { token_type, location, lexeme: None }
    }

    pub fn is_discardable(&self) -> bool {
        (self.token_type == TokenType::Comment
            || self.token_type == TokenType::Whitespace)
    }

    pub fn is_equality_comp(&self) -> bool {
        (self.token_type == TokenType::EqualEqual
            || self.token_type == TokenType::BangEqual)
    }

    pub fn is_order_comp(&self) -> bool {
        (self.token_type == TokenType::Greater
            || self.token_type == TokenType::GreaterEqual
            || self.token_type == TokenType::Less
            || self.token_type == TokenType::LessEqual)
    }

    pub fn is_arithmetic_comp(&self) -> bool {
        (self.token_type == TokenType::Plus
            || self.token_type == TokenType::Minus)
    }

    pub fn is_multiplicative_comp(&self) -> bool {
        (self.token_type == TokenType::Star
            || self.token_type == TokenType::Slash
            || self.token_type == TokenType::SlashSlash
            || self.token_type == TokenType::Percent)
    }

    pub fn is_unary_op(&self) -> bool {
        (self.token_type == TokenType::Bang
            || self.token_type == TokenType::Minus)
    }

    pub fn is_exponentiation(&self) -> bool {
        self.token_type == TokenType::Caret
    }
}
