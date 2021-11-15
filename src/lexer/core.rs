use std::collections::VecDeque;
use std::fmt;

use super::char_stream::CharStream;
use super::lex_numbers::*;
use super::lex_simple::*;
use super::lex_strings::*;

use crate::tokens::*;

pub fn lex(text: impl AsRef<str>) -> Result<Vec<Token>, LexerError> {
    let mut lexer = Lexer::new(text.as_ref());
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.next_token()?;
        if tok.kind == TokenKind::EndOfFile {
            break;
        }
        tokens.push(tok);
    }
    Ok(tokens)
}

#[derive(thiserror::Error, Debug)]
pub struct LexerError {
    pub position: Position,
    pub message: String,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", self.position, self.message)
    }
}

pub struct Lexer<'a> {
    stream: CharStream<'a>,
    indent_level: usize,
    brace_level: usize,
    queued: VecDeque<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer{ 
            stream: CharStream::new(input),
            indent_level: 0,
            brace_level: 0,
            queued: VecDeque::new(),
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        if let Some(tok) = self.queued.pop_front() {
            return Ok(tok);
        }

        if self.brace_level == 0 {
            self.stream.skip_while(|c| c.is_whitespace() && is_not_newline(c));
        } else {
            self.stream.skip_while(char::is_whitespace);
        }

        let start = self.stream.position();
        let c= if let Some(c) = self.stream.read_char() {
            c
        } else {
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
        };

        let kind = if let Some(kind) = self.match_parentheses(c) {
            kind
        } else if let Some(kind) = match_simple_operator(&mut self.stream, c) {
            kind
        } else if let Some(kind) = match_comment(&mut self.stream, c) {
            kind
        } else if let Some(kind) = match_identifier_or_keyword(&mut self.stream, c) {
            kind
        } else if let Some(kind) = match_number(&mut self.stream, c)? {
            kind
        } else if let Some(kind) = match_string(&mut self.stream, c)? {
            kind
        } else if let Some(tokens) = self.match_newline(c, start)? {
            #[allow(clippy::indexing_slicing)]
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
                message: "Unexpected fatal error with newline/indent parser".to_owned(),
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
                let _ = self.stream.read_if_char('\n');
            }
            while let Some(sc) = self.stream.read_if(|sc| sc == ' ' || sc == '\t') { 
                if sc == ' ' {
                    num_spaces += 1;
                } else if sc == 't' {
                    return Err(LexerError{
                        position: self.stream.position(),
                        message: "Tabs cannot be used as an indentation char".to_owned(),
                    })
                }
            }

            line_end_char = if let Some(nc) = self.stream.peek_char() {
                if is_newline(nc) {
                    let _ = self.stream.read_char();
                    nc
                } else {
                    break;
                }
            } else {
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
            };

            // This line ended up being empty -- reset
            num_spaces = 0;
            line_start = self.stream.position();
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
}

pub fn is_newline(c: char) -> bool {
    c == '\n' || c == '\r'
}

pub fn is_not_newline(c: char) -> bool {
    !is_newline(c)
}

#[cfg(test)]
mod tests {
    use crate::lexer::test_utils::*;

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
                TokenKind::Atom("foo1".into()),
                TokenKind::Indent,
                TokenKind::Atom("foo2".into()),
                TokenKind::Newline,
                TokenKind::Atom("foo3".into()),
                TokenKind::Indent,
                TokenKind::Atom("foo4".into()),
                TokenKind::Unindent,
                TokenKind::Unindent,
                TokenKind::Atom("foo5".into()),
                TokenKind::Indent,
                TokenKind::Atom("foo6".into()),
                TokenKind::Newline,
                TokenKind::Newline,
                TokenKind::Newline,
                TokenKind::Atom("foo7".into()),
                TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Atom("foo8".into()),
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
            TokenKind::Atom("foo1".into()),
            TokenKind::Indent,
            TokenKind::Atom("foo2".into()),
            TokenKind::Indent,
            TokenKind::Atom("foo3".into()),
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
                    kind: TokenKind::Atom("foo1".into()),
                    position: Position::new(0, 0, 0),
                },
                Token{
                    kind: TokenKind::Indent,
                    position: Position::new(0, 4, 4),
                },
                Token{
                    kind: TokenKind::Atom("foo2".into()),
                    position: Position::new(1, 4, 9),
                },
                Token{
                    kind: TokenKind::Indent,
                    position: Position::new(1, 8, 13),
                },
                Token{
                    kind: TokenKind::Atom("foo3".into()),
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