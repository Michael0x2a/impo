use core::str::Chars;
use std::iter::Peekable;
use std::collections::VecDeque;
use std::fmt;

use thiserror;

use crate::tokens::*;

#[derive(thiserror::Error, Debug)]
pub struct LexerError {
    position: Position,
    message: String,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", self.position, self.message)
    }
}

pub struct Lexer<'a> {
    stream: Peekable<Chars<'a>>,
    position: Position,
    indent_level: usize,
    brace_level: usize,
    queued: VecDeque<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer{ 
            stream: input.chars().peekable(),
            position: Position::start(),
            indent_level: 0,
            brace_level: 0,
            queued: VecDeque::new(),
        }
    }

    fn read_char(&mut self) -> Option<char> {
        let out = self.stream.next();
        if let Some(c) = out {
            self.position.advance(c);
        }
        out
    }

    fn peek_char(&mut self) -> Option<char> {
        self.stream.peek().map(char::to_owned)
    }

    fn read_if_char(&mut self, possible: char) -> bool {
        match self.peek_char() {
            Some(c) if c == possible => {
                let _ = self.read_char();
                true
            },
            _ => false,
        }
    }

    fn read_if(&mut self, filter: fn(char) -> bool) -> Option<char> {
        match self.peek_char() {
            Some(possible) if filter(possible) => {
                self.read_char()
            },
            _ => None,
        }
    }

    fn read_while(&mut self, filter: fn(char) -> bool) -> String {
        let mut out = String::new();
        while let Some(c) = self.read_if(filter) {
            out.push(c)
        }
        out
    }

    fn skip_while(&mut self, filter: fn(char) -> bool) -> usize {
        let mut num_skipped = 0;
        while let Some(_) = self.read_if(filter) { 
            num_skipped += 1;
        }
        num_skipped
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        if let Some(tok) = self.queued.pop_front() {
            return Ok(tok);
        }

        if self.brace_level == 0 {
            self.skip_while(|c| c.is_whitespace() && is_not_newline(c));
        } else {
            self.skip_while(char::is_whitespace);
        }

        let start = self.position.clone();
        let c= match self.read_char() {
            Some(c) => c,
            None => {
                // Handle the case where a file ends without a newline
                // (match_newline won't have an opportunity to detect
                // this case)
                let kind = if self.indent_level > 0 {
                    self.indent_level -= 1;
                    TokenKind::Unindent
                } else {
                    TokenKind::EndOfFile
                };
                return Ok(Token{
                    kind: kind,
                    position: start,
                })
            }
        };

        let kind = if let Some(kind) = self.match_parentheses(c) {
            kind
        } else if let Some(kind) = self.match_simple_operator(c) {
            kind
        } else if let Some(kind) = self.match_comment(c) {
            kind
        } else if let Some(kind) = self.match_identifier_or_keyword(c) {
            kind
        } else if let Some(tokens) = self.match_newline(c, start)? {
            for window in tokens.windows(2) {
                let curr = &window[0];
                let next = &window[1];
                if curr.kind == TokenKind::Newline && next.kind != TokenKind::Newline {
                    continue
                }
                self.queued.push_back(curr.clone());
            }

            let fatal_err = LexerError{
                position: start,
                message: format!("Unexpected fatal error with newline/indent parser"),
            };

            self.queued.push_back(match tokens.last() {
                Some(t) => t.clone(),
                None => { return Err(fatal_err) }
            });
            return Ok(match self.queued.pop_front() {
                Some(t) => t,
                None => { return Err(fatal_err) }
            });
        } else {
            return Err(LexerError{
                position: start,
                message: format!("Could not parse character '{}'", c),
            });
        };

        Ok(Token{
            position: start,
            kind: kind,
        })
    }

    fn match_parentheses(&mut self, c: char) -> Option<TokenKind> {
        let kind = match c {
            // Parentheses
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '[' => TokenKind::LSquare,
            ']' => TokenKind::RSquare,
            _ => {
                return None
            }
        };
        match kind {
            TokenKind::LParen | TokenKind::LBrace | TokenKind::LSquare => {
                self.brace_level += 1;
            },
            _ => {
                self.brace_level -= 1;
            } 
        }
        Some(kind)
    }

    fn match_simple_operator(&mut self, c: char) -> Option<TokenKind> {
        Some(match c {
            '+' => TokenKind::Plus,
            '-' => {
                if self.read_if_char('>') {
                    TokenKind::Arrow
                } else {
                    TokenKind::Minus
                }
            },
            '*' => TokenKind::Multiply,
            '/' => TokenKind::Divide,
            '>' => {
                if self.read_if_char('=') {
                    TokenKind::GreaterThanEquals
                } else {
                    TokenKind::GreaterThan
                }
            },
            '<' => {
                if self.read_if_char('=') {
                    TokenKind::LessThanEquals
                } else {
                    TokenKind::LessThan
                }
            },
            '=' => {
                if self.read_if_char('=') {
                    TokenKind::Equals
                } else {
                    TokenKind::Assign
                }
            },
            '!' => {
                if self.read_if_char('=') {
                    TokenKind::NotEquals
                } else {
                    TokenKind::Bang
                }
            },
            '|' => TokenKind::Pipe,
            '&' => TokenKind::Ampersand,
            '.' => TokenKind::Dot,
            ':' => TokenKind::Colon,
            ',' => TokenKind::Comma,
            _ => {
                return None;
            }
        })
    }

    fn match_comment(&mut self, c: char) -> Option<TokenKind> {
        if c != '#' {
            return None
        }
        let _ = self.read_if_char(' ');
        Some(TokenKind::Comment(self.read_while(is_not_newline)))
    }

    fn match_newline(&mut self, c: char, start: Position) -> Result<Option<Vec<Token>>, LexerError> {
        if self.brace_level != 0 || is_not_newline(c) {
            return Ok(None)
        }
        let mut acc = Vec::<Token>::new();

        // Gobble up empty lines, moving the effective line start ahead each time.
        let mut line_end_char = c;
        let mut line_start = start;
        let mut num_spaces = 0;
        loop {
            acc.push(Token { 
                kind: TokenKind::Newline, 
                position: line_start,
            });

            if line_end_char == '\r' {
                let _ = self.read_if_char('\n');
            }
            while let Some(sc) = self.read_if(|sc| sc == ' ' || sc == '\t') { 
                if sc == ' ' {
                    num_spaces += 1;
                } else if sc == 't' {
                    return Err(LexerError{
                        position: self.position.clone(),
                        message: format!("Tabs cannot be used as an indentation char"),
                    })
                }
            }

            line_end_char = match self.peek_char() {
                Some(nc) => {
                    if is_newline(nc) {
                        let _ = self.read_char();
                        nc
                    } else {
                        break;
                    }
                }
                None => {
                    // We are at the end of the file. Push any pending
                    // unindent tokens.
                    let unindent_tok = Token{
                        kind: TokenKind::Unindent,
                        position: line_start,
                    };
                    for _ in 0..self.indent_level {
                        acc.push(unindent_tok.clone());
                    }
                    self.indent_level = 0;
                    return Ok(Some(acc));
                }
            };

            // This line ended up being empty -- reset
            num_spaces = 0;
            line_start = self.position.clone();
        }

        // Use information from the first non-empty line to compute indentation levels
        if num_spaces % 4 != 0 {
            return Err(LexerError{
                position: line_start,
                message: format!("Indent contains {} spaces: must be a multiple of four", num_spaces),
            })
        }

        let indent_level = num_spaces / 4;
        if indent_level != self.indent_level {
            let (indent_kind, delta) = if indent_level > self.indent_level {
                (TokenKind::Indent, indent_level - self.indent_level)
            } else {
                (TokenKind::Unindent, self.indent_level - indent_level)
            };
            for _ in 0..delta {
                acc.push(Token { kind: indent_kind.clone(), position: line_start });
            }
        }
        self.indent_level = indent_level;

        Ok(Some(acc))
    }

    fn match_identifier_or_keyword(&mut self, c: char) -> Option<TokenKind> {
        if !is_identifier_start(c) {
            return None
        }

        let mut identifier = String::new();
        identifier.push(c);
        identifier.push_str(&self.read_while(is_identifier));

        Some(match identifier.as_str() {
            // Operator keywords
            "instanceof" => TokenKind::InstanceOf,
            "or" => TokenKind::Or,
            "and" => TokenKind::And,

            // Other keywords
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "return" => TokenKind::Return,
            "panic" => TokenKind::Panic,
            "fn" => TokenKind::Fn,
            "constructor" => TokenKind::Constructor,
            "interface" => TokenKind::Interface,
            "class" => TokenKind::Class,
            "sentinal" => TokenKind::Sentinal,
            "const" => TokenKind::Const,
            "implements" => TokenKind::Implements,
            _ => TokenKind::Identifier(identifier),
        })
    }
}

fn is_newline(c: char) -> bool {
    c == '\n' || c == '\r'
}

fn is_not_newline(c: char) -> bool {
    !is_newline(c)
}

fn is_identifier_start(c: char) -> bool {
    c == '_' || c.is_alphabetic()
}

fn is_identifier(c: char) -> bool {
    is_identifier_start(c) || c.is_numeric()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lexer_test<T: AsRef<Token>>(
        input: &str,
        expected_tokens: impl Iterator<Item = T>,
    ) -> Result<(), LexerError> {
        let mut last_position = Position::start();
        let mut lexer = Lexer::new(input);
        for (i, expected) in expected_tokens.enumerate() {
            let actual = lexer.next_token()?;
            assert_eq!(&actual, expected.as_ref(), "Mismatch at token index {}", i);
            last_position = expected.as_ref().position;
        }

        let expected_last_token = Token{
            kind: TokenKind::EndOfFile,
            position: last_position.add_horizontal(input.len() - last_position.offset),
        };

        assert_eq!(lexer.next_token()?, expected_last_token, "Mismatch at end");
        assert_eq!(lexer.next_token()?, expected_last_token, "Mismatch at end, 2nd attempt");

        Ok(())
    }

    fn lexer_test_ignore_positions<T: AsRef<TokenKind>>(
        input: &str,
        expected_token_kinds: impl Iterator<Item = T>,
    ) -> Result<(), LexerError> {
        let mut lexer = Lexer::new(input);
        for (i, expected_kind) in expected_token_kinds.enumerate() {
            let actual = lexer.next_token()?;
            assert_eq!(&actual.kind, expected_kind.as_ref(), "Mismatch at token index {}", i);
        }

        let last = lexer.next_token()?;
        assert_eq!(last.kind, TokenKind::EndOfFile, "Mismatch at end");
        assert_eq!(last.position.offset, input.len(), "Mismatch at end");

        Ok(())
    }

    #[test]
    fn test_lex_single_char_operators() -> Result<(), LexerError> {
        let input = "+ - * / < > ! | & . = : ,";
        let expected_kinds = vec![
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Multiply,
            TokenKind::Divide,
            TokenKind::LessThan,
            TokenKind::GreaterThan,
            TokenKind::Bang,
            TokenKind::Pipe,
            TokenKind::Ampersand,
            TokenKind::Dot,
            TokenKind::Assign,
            TokenKind::Colon,
            TokenKind::Comma,
        ];
        lexer_test(
            input,
            expected_kinds.iter().enumerate().map(|(index, kind)| {
                Token{
                    kind: kind.clone(),
                    position: Position::new(0, index * 2, index * 2),
                }
            }),
        )
    }

    #[test]
    fn test_lex_double_char_operators() -> Result<(), LexerError> {
        let input = "== != <= >= ->";
        let expected_kinds = vec![
            TokenKind::Equals,
            TokenKind::NotEquals,
            TokenKind::LessThanEquals,
            TokenKind::GreaterThanEquals,
            TokenKind::Arrow,
        ];
        lexer_test(
            input,
            expected_kinds.iter().enumerate().map(|(index, kind)| {
                Token{
                    kind: kind.clone(),
                    position: Position::new(0, index * 3, index * 3),
                }
            }),
        )
    }

    #[test]
    fn test_lex_comments() -> Result<(), LexerError> {
        lexer_test(
        concat!(
                "# comment 1\n",
                "# comment # with # extra # hash\r\n",
                "# final comment\r",
            ),
            vec![
                Token{
                    kind: TokenKind::Comment("comment 1".to_owned()),
                    position: Position::new(0, 0, 0),
                },
                Token{
                    kind: TokenKind::Newline,
                    position: Position::new(0, 11, 11),
                },
                Token{
                    kind: TokenKind::Comment("comment # with # extra # hash".to_owned()),
                    position: Position::new(1, 0, 12),
                },
                Token{
                    kind: TokenKind::Newline,
                    position: Position::new(1, 31, 43),
                },
                Token{
                    kind: TokenKind::Comment("final comment".to_owned()),
                    position: Position::new(2, 0, 45),
                },
                Token{
                    kind: TokenKind::Newline,
                    position: Position::new(2, 15, 60),
                },
            ].iter(),
        )
    }

    #[test]
    fn test_lex_identifiers() -> Result<(), LexerError> {
        lexer_test(
            "for and implements foo b12 andvar",
            vec![
                Token{
                    kind: TokenKind::For,
                    position: Position::new(0, 0, 0),
                },
                Token{
                    kind: TokenKind::And,
                    position: Position::new(0, 4, 4),
                },
                Token{
                    kind: TokenKind::Implements,
                    position: Position::new(0, 8, 8),
                },
                Token{
                    kind: TokenKind::Identifier("foo".to_owned()),
                    position: Position::new(0, 19, 19),
                },
                Token{
                    kind: TokenKind::Identifier("b12".to_owned()),
                    position: Position::new(0, 23, 23),
                },
                Token{
                    kind: TokenKind::Identifier("andvar".to_owned()),
                    position: Position::new(0, 27, 27),
                },
            ].iter(),
        )
    }

    #[test]
    fn test_indentation() -> Result<(), LexerError> {
        lexer_test_ignore_positions(
            concat!(
                "foo1\n",
                "    foo2\n",
                "    foo3\n",
                "        foo4\n",
                "foo5\n",
                "    foo6\n",
                "      \n",
                "\n",
                "    foo7\n",
                "\n",
                "        foo8\n",
            ),
            vec![
                TokenKind::Identifier("foo1".to_owned()),
                TokenKind::Indent,
                TokenKind::Identifier("foo2".to_owned()),
                TokenKind::Newline,
                TokenKind::Identifier("foo3".to_owned()),
                TokenKind::Indent,
                TokenKind::Identifier("foo4".to_owned()),
                TokenKind::Unindent,
                TokenKind::Unindent,
                TokenKind::Identifier("foo5".to_owned()),
                TokenKind::Indent,
                TokenKind::Identifier("foo6".to_owned()),
                TokenKind::Newline,
                TokenKind::Newline,
                TokenKind::Newline,
                TokenKind::Identifier("foo7".to_owned()),
                TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Identifier("foo8".to_owned()),
                TokenKind::Unindent,
                TokenKind::Unindent,
            ].iter(),
        )
    }

    #[test]
    fn test_indentation_endings() -> Result<(), LexerError> {
        struct TestCase {
            description: &'static str,
            input: &'static str,
        }

        let test_cases = vec![
            TestCase{
                description: "Standard input",
                input: "foo1\n    foo2\n        foo3\n",
            },
            TestCase{
                description: "No newline at end",
                input: "foo1\n    foo2\n        foo3",
            },
            TestCase{
                description: "No newline at end; extra spaces",
                input: "foo1\n    foo2\n        foo3  ",
            },
            TestCase{
                description: "Extra spaces before newlines",
                input: "foo1  \n    foo2  \n        foo3  \n",
            },
            TestCase{
                description: "Extra spaces at end",
                input: "foo1\n    foo2\n        foo3\n  ",
            },
        ];

        let expected = vec![
            TokenKind::Identifier("foo1".to_owned()),
            TokenKind::Indent,
            TokenKind::Identifier("foo2".to_owned()),
            TokenKind::Indent,
            TokenKind::Identifier("foo3".to_owned()),
            TokenKind::Unindent,
            TokenKind::Unindent,
        ];

        for test_case in test_cases {
            println!("Running test case: {}", test_case.description);
            lexer_test_ignore_positions(
                test_case.input,
                expected.iter(),
            )?;
        }

        Ok(())
    }

    #[test]
    fn test_indentation_ending_with_newline() -> Result<(), LexerError> {
        lexer_test(
            concat!(
                "foo1\n",
                "    foo2\n",
                "        foo3\n",
                "\n",
                "\n",
            ),
            vec![
                Token{
                    kind: TokenKind::Identifier("foo1".to_owned()),
                    position: Position::new(0, 0, 0),
                },
                Token{
                    kind: TokenKind::Indent,
                    position: Position::new(0, 4, 4),
                },
                Token{
                    kind: TokenKind::Identifier("foo2".to_owned()),
                    position: Position::new(1, 4, 9),
                },
                Token{
                    kind: TokenKind::Indent,
                    position: Position::new(1, 8, 13),
                },
                Token{
                    kind: TokenKind::Identifier("foo3".to_owned()),
                    position: Position::new(2, 8, 22),
                },
                Token{
                    kind: TokenKind::Newline,
                    position: Position::new(2, 12, 26),
                },
                Token{
                    kind: TokenKind::Newline,
                    position: Position::new(4, 0, 28),
                },
                Token{
                    kind: TokenKind::Unindent,
                    position: Position::new(5, 0, 29),
                },
                Token{
                    kind: TokenKind::Unindent,
                    position: Position::new(5, 0, 29),
                },
            ].into_iter()
        )
    }
}