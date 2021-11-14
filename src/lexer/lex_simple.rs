use super::char_stream::CharStream;
use super::core::is_not_newline;
use crate::tokens::*;

pub fn match_simple_operator(stream: &mut CharStream, c: char) -> Option<TokenKind> {
    Some(match c {
        '+' => TokenKind::Plus,
        '-' => {
            if stream.read_if_char('>') {
                TokenKind::Arrow
            } else {
                TokenKind::Minus
            }
        },
        '*' => TokenKind::Multiply,
        '/' => TokenKind::Divide,
        '%' => TokenKind::Percent,
        '>' => {
            if stream.read_if_char('=') {
                TokenKind::GreaterThanEquals
            } else {
                TokenKind::GreaterThan
            }
        },
        '<' => {
            if stream.read_if_char('=') {
                TokenKind::LessThanEquals
            } else {
                TokenKind::LessThan
            }
        },
        '=' => {
            if stream.read_if_char('=') {
                TokenKind::Equals
            } else {
                TokenKind::Assign
            }
        },
        '!' => {
            if stream.read_if_char('=') {
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

pub fn match_comment(stream: &mut CharStream, c: char) -> Option<TokenKind> {
    if c != '#' {
        return None
    }
    let _ = stream.read_if_char(' ');
    Some(TokenKind::Comment(stream.read_while(is_not_newline)))
}

pub fn match_identifier_or_keyword(stream: &mut CharStream, c: char) -> Option<TokenKind> {
    if !is_identifier_start(c) {
        return None
    }

    let mut identifier = String::new();
    identifier.push(c);
    identifier.push_str(&stream.read_while(is_identifier));

    Some(match identifier.as_str() {
        // Operator keywords
        "instanceof" => TokenKind::InstanceOf,
        "or" => TokenKind::Or,
        "and" => TokenKind::And,

        // Other keywords
        "if" => TokenKind::If,
        "elif" => TokenKind::Elif,
        "else" => TokenKind::Else,
        "for" => TokenKind::For,
        "from" => TokenKind::From,
        "to" => TokenKind::To,
        "foreach" => TokenKind::Foreach,
        "in" => TokenKind::In,
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
        "true" => TokenKind::BoolLiteral(true),
        "false" => TokenKind::BoolLiteral(false),
        _ => TokenKind::Atom(identifier.into()),
    })
}

fn is_identifier_start(c: char) -> bool {
    c == '_' || c.is_alphabetic()
}

fn is_identifier(c: char) -> bool {
    is_identifier_start(c) || c.is_numeric()
}

#[cfg(test)]
mod tests {
    use crate::lexer::test_utils::*;

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
                    kind: TokenKind::Atom("foo".into()),
                    position: Position::new(0, 19, 19),
                },
                Token{
                    kind: TokenKind::Atom("b12".into()),
                    position: Position::new(0, 23, 23),
                },
                Token{
                    kind: TokenKind::Atom("andvar".into()),
                    position: Position::new(0, 27, 27),
                },
            ].iter(),
        )
    }
}