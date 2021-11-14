use super::char_stream::CharStream;
use super::core::LexerError;
use crate::tokens::*;
use crate::values::{IntLiteral, FloatLiteral};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum NumState {
    InitialZero,
    IntegralDigits,
    FractionalDigits,
    SciNotationStart,
    SciNotationDigits,
}

fn make_bad_initial_zero_err(nc: char, pos: Position) -> Result<Option<TokenKind>, LexerError> {
    Err(LexerError{
        message: format!("Initial 0 must be followed by x, o, or b; followed by '{}' instead", nc),
        position: pos,
    })
}

fn make_nonstandard_base_into_floating_point_err(pos: Position) -> Result<Option<TokenKind>, LexerError> {
    Err(LexerError{
        message: "Cannot prefix floating point numbers with 0x, 0o, or 0b".to_owned(),
        position: pos,
    })
}

fn make_bad_sci_notation_transition_err(seq: &str, pos: Position) -> Result<Option<TokenKind>, LexerError> {
    Err(LexerError{
        message: format!("Scientific notation contained invalid char '{}'", seq),
        position: pos,
    })
}

#[allow(clippy::too_many_lines)]
pub fn match_number(stream: &mut CharStream, c: char) -> Result<Option<TokenKind>, LexerError> {
    if !is_digit(c) {
        return Ok(None)
    }
    let mut base = 10;
    let mut integral_digits = String::new();
    let mut fractional_digits = String::new();
    let mut sci_notation_digits = String::new();

    let mut state = NumState::InitialZero;
    if c != '0' {
        integral_digits.push(c);
        state = NumState::IntegralDigits;
    }
    loop {
        state = match state {
            NumState::InitialZero => {
                match stream.peek_char() {
                    Some('.') => {
                        let _ = stream.read_char();
                        integral_digits.push('0');
                        NumState::FractionalDigits
                    }
                    Some(nc) if nc.is_ascii_alphabetic() => {
                        let _ = stream.read_char();
                        base = match nc {
                            'x' => 16,
                            'o' => 8,
                            'b' => 2,
                            _ => {
                                return make_bad_initial_zero_err(nc, stream.position());
                            }
                        };
                        NumState::IntegralDigits
                    }
                    Some(nc) if is_digit(nc) => {
                        return make_bad_initial_zero_err(nc, stream.position());
                    },
                    _ => {
                        integral_digits.push('0');
                        break
                    }
                }
            },
            NumState::IntegralDigits => {
                match stream.peek_char() {
                    Some(nc) if is_acceptable_digit(nc, base) => {
                        let _ = stream.read_char();
                        integral_digits.push(nc);
                        NumState::IntegralDigits
                    },
                    Some('.') => {
                        if base != 10 {
                            return make_nonstandard_base_into_floating_point_err(stream.position());
                        }
                        let _ = stream.read_char();
                        NumState::FractionalDigits
                    },
                    _ => {
                        break
                    }
                }
            },
            NumState::FractionalDigits => {
                match stream.peek_char() {
                    Some(nc) if is_digit(nc) => {
                        let _ = stream.read_char();
                        fractional_digits.push(nc);
                        NumState::FractionalDigits
                    },
                    Some('e') => {
                        let _ = stream.read_char();
                        NumState::SciNotationStart
                    },
                    _ => {
                        break
                    }
                }
            },
            NumState::SciNotationStart => {
                match stream.peek_char() {
                    Some('-') => {
                        let _ = stream.read_char();
                        sci_notation_digits.push('-');
                        NumState::SciNotationDigits
                    },
                    Some(nc) if is_digit(nc) => {
                        NumState::SciNotationDigits
                    },
                    Some(nc) => {
                        return make_bad_sci_notation_transition_err(&nc.to_string(), stream.position());
                    }
                    None => {
                        return make_bad_sci_notation_transition_err("EOF", stream.position());
                    }
                }

            },
            NumState::SciNotationDigits => {
                match stream.peek_char() {
                    Some(nc) if is_digit(nc) => {
                        let _ = stream.read_char();
                        sci_notation_digits.push(nc);
                        NumState::SciNotationDigits
                    },
                    _ => {
                        break
                    }
                }
            },
        }
    }

    let kind = if fractional_digits.is_empty() {
        TokenKind::IntLiteral(IntLiteral{
            base: base,
            digits: integral_digits.into(),
        })
    } else {
        TokenKind::FloatLiteral(FloatLiteral{
            integral_digits: integral_digits.into(),
            fractional_digits: fractional_digits.into(),
            power: sci_notation_digits.into(),
        })
    };
    Ok(Some(kind))
}

fn is_acceptable_digit(c: char, base: usize) -> bool {
    match base {
        10 => is_digit(c),
        16 => is_hex_digit(c),
        8 => is_octal_digit(c),
        2 => is_binary_digit(c),
        _ => false,
    }
}

fn is_digit(c: char) -> bool {
    matches!(c, '0'..='9')
}

fn is_hex_digit(c: char) -> bool {
    matches!(c, '0'..='9' | 'a'..='f' | 'A'..='F')
}

fn is_octal_digit(c: char) -> bool {
    matches!(c, '0'..='7')
}

fn is_binary_digit(c: char) -> bool {
    matches!(c, '0'..='1')
}

#[cfg(test)]
mod tests {
    use crate::lexer::test_utils::*;

    #[test]
    fn test_number() -> Result<(), LexerError> {
        lexer_test_ignore_positions(
            
            concat!(
                "0 ",
                "0.0 ",
                "1234567890 ",
                "0xabcdef19 ",
                "0o12345670 ",
                "0b10101010 ",
                "0.12345678 ",
                "123.456789 ",
                "1.3e123456 ",
                "1.3e-12345 ",
            ),
            vec![
                TokenKind::IntLiteral(IntLiteral{
                    base: 10,
                    digits: "0".into(),
                }),
                TokenKind::FloatLiteral(FloatLiteral{
                    integral_digits: "0".into(),
                    fractional_digits: "0".into(),
                    power: "".into(),
                }),
                TokenKind::IntLiteral(IntLiteral{
                    base: 10,
                    digits: "1234567890".into(),
                }),
                TokenKind::IntLiteral(IntLiteral{
                    base: 16,
                    digits: "abcdef19".into(),
                }),
                TokenKind::IntLiteral(IntLiteral{
                    base: 8,
                    digits: "12345670".into(),
                }),
                TokenKind::IntLiteral(IntLiteral{
                    base: 2,
                    digits: "10101010".into(),
                }),
                TokenKind::FloatLiteral(FloatLiteral{
                    integral_digits: "0".into(),
                    fractional_digits: "12345678".into(),
                    power: "".into(),
                }),
                TokenKind::FloatLiteral(FloatLiteral{
                    integral_digits: "123".into(),
                    fractional_digits: "456789".into(),
                    power: "".into(),
                }),
                TokenKind::FloatLiteral(FloatLiteral{
                    integral_digits: "1".into(),
                    fractional_digits: "3".into(),
                    power: "123456".into(),
                }),
                TokenKind::FloatLiteral(FloatLiteral{
                    integral_digits: "1".into(),
                    fractional_digits: "3".into(),
                    power: "-12345".into(),
                }),
            ].into_iter()
        )
    }
}
