use super::char_stream::CharStream;
use super::core::LexerError;
use crate::tokens::*;

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
        message: format!("Cannot prefix floating point numbers with 0x, 0o, or 0b"),
        position: pos,
    })
}

fn make_bad_sci_notation_transition_err(seq: &str, pos: Position) -> Result<Option<TokenKind>, LexerError> {
    Err(LexerError{
        message: format!("Scientific notation contained invalid char '{}'", seq),
        position: pos,
    })
}

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
        TokenKind::IntLiteral{
            base: base,
            digits: integral_digits,
        }
    } else {
        TokenKind::FloatLiteral{
            integral_digits: integral_digits,
            fractional_digits: fractional_digits,
            power: sci_notation_digits,
        }
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
    match c {
        '0'..='9' => true,
        _ => false,
    }
}

fn is_hex_digit(c: char) -> bool {
    match c {
        '0'..='9' => true,
        'a'..='f' => true,
        'A'..='F' => true,
        _ => false,
    }
}

fn is_octal_digit(c: char) -> bool {
    match c {
        '0'..='7' => true,
        _ => false,
    }
}

fn is_binary_digit(c: char) -> bool {
    match c {
        '0'..='1' => true,
        _ => false,
    }
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
                TokenKind::IntLiteral{
                    base: 10,
                    digits: "0".to_owned(),
                },
                TokenKind::FloatLiteral{
                    integral_digits: "0".to_owned(),
                    fractional_digits: "0".to_owned(),
                    power: "".to_owned(),
                },
                TokenKind::IntLiteral{
                    base: 10,
                    digits: "1234567890".to_owned(),
                },
                TokenKind::IntLiteral{
                    base: 16,
                    digits: "abcdef19".to_owned(),
                },
                TokenKind::IntLiteral{
                    base: 8,
                    digits: "12345670".to_owned(),
                },
                TokenKind::IntLiteral{
                    base: 2,
                    digits: "10101010".to_owned(),
                },
                TokenKind::FloatLiteral{
                    integral_digits: "0".to_owned(),
                    fractional_digits: "12345678".to_owned(),
                    power: "".to_owned(),
                },
                TokenKind::FloatLiteral{
                    integral_digits: "123".to_owned(),
                    fractional_digits: "456789".to_owned(),
                    power: "".to_owned(),
                },
                TokenKind::FloatLiteral{
                    integral_digits: "1".to_owned(),
                    fractional_digits: "3".to_owned(),
                    power: "123456".to_owned(),
                },
                TokenKind::FloatLiteral{
                    integral_digits: "1".to_owned(),
                    fractional_digits: "3".to_owned(),
                    power: "-12345".to_owned(),
                },
            ].into_iter()
        )
    }
}
