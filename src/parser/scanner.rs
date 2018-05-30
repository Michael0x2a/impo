use std::iter::Peekable;
use std::str::Chars;
use std::mem;
use text::{TextOffset, TextRange};
use parser::tokens::{TokenType, Token};
use errors::{ImpoError, ErrorStage};

pub struct TokenStream<'src, 'dst> {
    source: Peekable<Chars<'src>>,
    tokens: &'dst mut Vec<Token>,
    start: usize,
    current: usize,
    buffer: String,
}

impl<'src, 'dst> TokenStream<'src, 'dst> {
    pub fn new(source: Chars<'src>, tokens: &'dst mut Vec<Token>) -> TokenStream<'src, 'dst> {
        TokenStream {
            source: source.peekable(),
            tokens,
            start: 0,
            current: 0,
            buffer: String::new(),
        }
    }

    pub fn scan(mut self) -> Result<(), ImpoError> {
        let mut chars_left = true;
        while chars_left {
            let tok = self.scan_token()?;
            chars_left = tok.token_type != TokenType::Eof;
            if !tok.is_discardable() {
                self.tokens.push(tok);
            }
        }
        Ok(())
    }

    fn get_range(&self) -> TextRange {
        TextRange::new_absolute(
            TextOffset::new(self.start),
            TextOffset::new(self.current),
        )
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        let mut new_buffer = String::new();
        mem::swap(&mut self.buffer, &mut new_buffer);

        let range = self.get_range();
        self.start = self.current;
        Token::new(
            token_type,
            range,
            new_buffer,
        )
    }

    fn make_empty_token(&mut self, token_type: TokenType) -> Token {
        self.buffer.clear();
        self.start = self.current;
        Token::new_empty(
            token_type,
            self.get_range(),
        )
    }

    fn make_error(&self, message: &str) -> ImpoError {
        ImpoError::new(self.get_range(), ErrorStage::Tokenizing, message)
    }

    fn discard_previous(&mut self) -> Result<(), ImpoError> {
        if self.buffer.is_empty() {
            Err(self.make_error("Cannot discard previous; buffer is empty"))
        } else {
            self.buffer.pop();
            Ok(())
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        match self.source.next() {
            Some(c) => {
                self.buffer.push(c);
                Some(c)
            },
            None => None,
        }
    }

    fn matches(&mut self, expected: char) -> bool {
        let result = match self.peek() {
            Some(c) if *c == expected => true,
            _ => false,
        };
        if result {
            self.advance().unwrap();
        }
        result
    }

    fn match_while<F>(&mut self, pred: F) -> Option<char>
        where F: Fn(char) -> bool {
        loop {
            let c = match self.peek() {
                Some(n) => *n,
                None => { return None; }
            };
            if pred(c) {
                self.advance().unwrap();
            } else {
                return Some(c)
            }
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    fn select_if(&mut self, expected: char, match_type: TokenType, no_match_type: TokenType) -> TokenType {
        if self.matches(expected) { match_type } else { no_match_type }
    }

    fn scan_token(&mut self) -> Result<Token, ImpoError> {
        let c = match self.advance() {
            Some(c) => c,
            None => {
                return Ok(self.make_empty_token(TokenType::Eof));
            }
        };

        let token_type = match c {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => self.select_if(
                '>',
                TokenType::Arrow,
                TokenType::Minus),
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            ':' => TokenType::Colon,
            '*' => TokenType::Star,
            '/' => self.select_if(
                '/',
                TokenType::SlashSlash,
                TokenType::Slash),
            '%' => TokenType::Percent,
            '^' => TokenType::Caret,
            '?' => TokenType::QuestionMark,
            '!' => self.select_if(
                '=',
                TokenType::BangEqual,
                TokenType::Bang,
            ),
            '=' => self.select_if(
                '=',
                TokenType::EqualEqual,
                TokenType::Equal,
            ),
            '<' => self.select_if(
                '=',
                TokenType::LessEqual,
                TokenType::Less,
            ),
            '>' => self.select_if(
                '=',
                TokenType::GreaterEqual,
                TokenType::Greater,
            ),
            '"' => self.match_string_literal()?,
            '#' => {
                self.match_while(|p| p != '\n' && p != '\r').unwrap();
                TokenType::Comment
            },
            '\n' => TokenType::Newline,
            '\r' => {
                self.matches('\n');
                TokenType::Newline
            },
            ' ' | '\t'  => TokenType::Whitespace,
            c if is_digit(c) => self.match_number_literal()?,
            c if c.is_alphabetic() => self.match_identifier()?,
            _ => TokenType::UnexpectedSequence,
        };
        Ok(self.make_token(token_type))
    }

    fn match_string_literal(&mut self) -> Result<TokenType, ImpoError> {
        // Discard first quotation mark
        self.discard_previous()?;

        let mut ignore_next = false;
        loop {
            let c = match self.advance() {
                Some(c) => c,
                None => {
                    return Err(self.make_error("Unexpected EOF when parsing string literal"));
                }
            };
            if ignore_next {
                ignore_next = false;
            } else if c == '"' {
                break;
            } else if c == '\\' {
                self.discard_previous()?;
                ignore_next = true;
            }
        }
        // Last quotation mark
        self.discard_previous()?;
        Ok(TokenType::StringLiteral)
    }

    fn match_number_literal(&mut self) -> Result<TokenType, ImpoError> {
        let mut is_float = false;
        let mut prev_is_dot = false;
        loop {
            let upcoming = match self.peek() {
                Some(c) => *c,
                None => {
                    break;
                }
            };
            if is_digit(upcoming) {
                prev_is_dot = false;
                self.advance().unwrap();
            } else if upcoming == '.' {
                if is_float {
                    // We've already encountered the decimal point.
                    prev_is_dot = false;
                    break;
                } else {
                    is_float = true;
                    prev_is_dot = true;
                    self.advance().unwrap();
                }
            } else {
                break;
            }
        };
        if prev_is_dot {
            Err(self.make_error("Unexpected end of float literal"))
        } else if is_float {
            Ok(TokenType::FloatLiteral)
        } else {
            Ok(TokenType::IntLiteral)
        }
    }

    fn match_identifier(&mut self) -> Result<TokenType, ImpoError> {
        self.match_while(|p| p.is_alphabetic() || is_digit(p)).unwrap();
        let token_type = match self.buffer.as_str() {
            "and" => TokenType::And,
            "or" => TokenType::Or,
            "class" => TokenType::Class,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "fn" => TokenType::Fn,
            "while" => TokenType::While,
            "repeat" => TokenType::Repeat,
            "nil" => TokenType::Nil,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "var" => TokenType::Var,
            "dyn" => TokenType::Dyn,
            _ => TokenType::Identifier,
        };

        Ok(token_type)
    }
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

