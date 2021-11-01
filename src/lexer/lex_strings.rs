use super::char_stream::CharStream;
use super::core::LexerError;
use crate::tokens::*;

const QUOTE_CHAR: char = '"';
const ESCAPE_START: char = '\\';

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum StringState {
    RegularChar,
    Escape,
}

pub fn match_string(stream: &mut CharStream, c: char) -> Result<Option<TokenKind>, LexerError> {
    if c != QUOTE_CHAR {
        return Ok(None);
    }

    let mut output = String::new();
    let mut state = StringState::RegularChar;
    loop {
        let pos = stream.position();
        let curr = stream
            .read_char()
            .ok_or(make_unexpected_end_of_string_err(pos))?;
        state = match state {
            StringState::RegularChar => {
                match curr {
                    ESCAPE_START => StringState::Escape,
                    QUOTE_CHAR => {
                        break
                    },
                    _ => {
                        output.push(curr);
                        StringState::RegularChar
                    }
                }
            },
            StringState::Escape => {
                output.push(escape_symbol_to_char(curr));
                StringState::RegularChar
            },
        }
    }

    Ok(Some(TokenKind::StringLiteral(output)))
}

fn make_unexpected_end_of_string_err(pos: Position) -> LexerError {
    LexerError{
        message: format!("Unexpected end of string"),
        position: pos,
    }
}

fn escape_symbol_to_char(symbol: char) -> char {
    match symbol {
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        _ => symbol,
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::test_utils::*;

    #[test]
    fn test_string() -> Result<(), LexerError> {
        lexer_test(
            concat!(
                r#""hello world" "#,
                r#""\n\r\t\\\"\'\q" "#,
                r#""😀" "#,
                r#""" "#,
            ),
            vec![
                Token{
                    position: Position::new(0, 0, 0),
                    kind: TokenKind::StringLiteral("hello world".to_owned()),
                },
                Token{
                    position: Position::new(0, 14, 14),
                    kind: TokenKind::StringLiteral("\n\r\t\\\"\'q".to_owned()),
                },
                Token{
                    position: Position::new(0, 31, 31),
                    kind: TokenKind::StringLiteral("😀".to_owned()),
                },
                Token{
                    position: Position::new(0, 35, 35),
                    kind: TokenKind::StringLiteral("".to_owned()),
                },
            ].into_iter()
        )
    }
}