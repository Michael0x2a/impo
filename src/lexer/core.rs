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
    if let Some(last) = tokens.last() {
        if last.kind != TokenKind::Newline {
            let extra_newline = Token{
                kind: TokenKind::Newline,
                position: last.end_position(),
            };
            tokens.push(extra_newline);
        }
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

pub struct Lexer {
    stream: CharStream,
    indent_level: usize,
    brace_level: usize,
    queued: VecDeque<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
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
            for token in tokens {
                self.queued.push_back(token);
            }
            let fatal_err = LexerError{
                position: start,
                message: "Unexpected fatal error with newline/indent parser".to_owned(),
            };
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
        self.brace_level = match kind {
            TokenKind::LParen | TokenKind::LBrace | TokenKind::LSquare => {
                self.brace_level.saturating_add(1)
            },
            _ => self.brace_level.saturating_sub(1)
        };
        Some(kind)
    }

    fn match_newline(&mut self, c: char, original_start: Position) -> Result<Option<Vec<Token>>, LexerError> {
        if self.brace_level != 0 || is_not_newline(c) {
            return Ok(None)
        }
        let mut acc = Vec::<Token>::new();

        // Gobble up empty lines, moving the effective line start ahead each time.
        let mut line_end_char = c;
        let mut curr_line_start = original_start;
        let new_indent_level;

        loop {
            // Push newline for current line
            acc.push(Token { 
                kind: TokenKind::Newline, 
                position: curr_line_start,
            });

            // Compute indentation level for next line
            if line_end_char == '\r' {
                let _ = self.stream.read_if_char('\n');
            }
            let mut num_spaces = 0;
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

            // Determine if we need to move on to the next line
            line_end_char = if let Some(nc) = self.stream.peek_char() {
                if is_newline(nc) {
                    // Mark the current newline position, then consume and
                    // advance to the next line.
                    curr_line_start = self.stream.position();
                    let _ = self.stream.read_char();
                    nc
                } else {
                    // Current line is non-empty. End and handle indents/unindents
                    if num_spaces % 4 != 0 {
                        return Err(LexerError{
                            position: curr_line_start,
                            message: format!("Indent contains {} spaces: must be a multiple of four", num_spaces),
                        })
                    }
                    new_indent_level = num_spaces / 4;
                    break;
                }
            } else {
                // File is over. End and handle any remaining unindents
                new_indent_level = 0;
                break;
            };
        }

        if new_indent_level != self.indent_level {
            if new_indent_level > self.indent_level {
                // Indents always go at the end. This ensures empty lines belong to the current block
                let delta = new_indent_level - self.indent_level;
                for _ in 0..delta {
                    acc.push(Token { kind: TokenKind::Indent, position: curr_line_start });
                }
            } else {
                // Invariant: the list always contains at least one newline
                let (first, rest) = acc.split_first().ok_or(LexerError{
                    message: "Unexpected fatal error: newline lexer did not find any newlines".to_owned(),
                    position: original_start,
                })?;

                // Dedents go after the first newline. This ensures empty lines belong to the new block
                let delta = self.indent_level - new_indent_level;
                let mut new_acc = vec![first.clone()];
                for _ in 0..delta {
                    new_acc.push(Token{kind: TokenKind::Unindent, position: original_start});
                }
                new_acc.extend_from_slice(rest);
                acc = new_acc;
            };
        }
        self.indent_level = new_indent_level;

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
                "\n",
                "        foo8\n",
                "\n",
                "\n",
                "    foo9\n",
            ),
            vec![
                TokenKind::Atom("foo1".into()),
                TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Atom("foo2".into()),
                TokenKind::Newline,
                TokenKind::Atom("foo3".into()),
                TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Atom("foo4".into()),
                TokenKind::Newline,
                TokenKind::Unindent,
                TokenKind::Unindent,
                TokenKind::Atom("foo5".into()),
                TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Atom("foo6".into()),
                TokenKind::Newline,
                TokenKind::Newline,
                TokenKind::Newline,
                TokenKind::Atom("foo7".into()),
                TokenKind::Newline,
                TokenKind::Newline,
                TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Atom("foo8".into()),
                TokenKind::Newline,
                TokenKind::Unindent,
                TokenKind::Newline,
                TokenKind::Newline,
                TokenKind::Atom("foo9".into()),
                TokenKind::Newline,
                TokenKind::Unindent,
            ].iter(),
        )
    }

    #[test]
    fn test_indentation_endings_single_newline() -> Result<(), LexerError> {
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
            TokenKind::Newline,
            TokenKind::Indent,
            TokenKind::Atom("foo2".into()),
            TokenKind::Newline,
            TokenKind::Indent,
            TokenKind::Atom("foo3".into()),
            TokenKind::Newline,
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
    fn test_indentation_endings_no_newline() -> Result<(), LexerError> {
        struct TestCase {
            description: &'static str,
            input: &'static str,
        }

        let test_cases = vec![
            TestCase{
                description: "No newline at end",
                input: "foo1\n    foo2\n        foo3",
            },
            TestCase{
                description: "No newline at end; extra spaces",
                input: "foo1\n    foo2\n        foo3  ",
            },
        ];

        let expected = vec![
            TokenKind::Atom("foo1".into()),
            TokenKind::Newline,
            TokenKind::Indent,
            TokenKind::Atom("foo2".into()),
            TokenKind::Newline,
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
    fn test_indentation_ending_with_many_newline() -> Result<(), LexerError> {
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
                    kind: TokenKind::Newline,
                    position: Position::new(0, 4, 4),
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
                    kind: TokenKind::Newline,
                    position: Position::new(1, 8, 13),
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
                    kind: TokenKind::Unindent,
                    position: Position::new(2, 12, 26),
                },
                Token{
                    kind: TokenKind::Unindent,
                    position: Position::new(2, 12, 26),
                },
                Token{
                    kind: TokenKind::Newline,
                    position: Position::new(3, 0, 27),
                },
                Token{
                    kind: TokenKind::Newline,
                    position: Position::new(4, 0, 28),
                },
                Token{
                    kind: TokenKind::EndOfFile,
                    position: Position::new(5, 0, 29),
                },
            ].into_iter()
        )
    }
}