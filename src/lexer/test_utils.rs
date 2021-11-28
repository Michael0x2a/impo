use super::core::Lexer;

pub use crate::tokens::{Token, TokenKind, Position};
pub use crate::values::{IntLiteral, FloatLiteral};
pub use super::core::LexerError;

pub fn lexer_test<T: AsRef<Token>>(
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

    let length = input.chars().count();
    let expected_last_token = Token{
        kind: TokenKind::EndOfFile,
        position: last_position.add_horizontal(length - last_position.offset),
    };

    assert_eq!(lexer.next_token()?, expected_last_token, "Mismatch at end");
    assert_eq!(lexer.next_token()?, expected_last_token, "Mismatch at end, 2nd attempt");

    Ok(())
}

pub fn lexer_test_ignore_positions<T: AsRef<TokenKind>>(
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