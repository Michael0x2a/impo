use anyhow::Result;
use impo::{lexer::Lexer, tokens::TokenKind};

fn main() -> Result<()> {
    let mut lexer = Lexer::new("# test");
    loop {
        let tok = lexer.next_token()?;
        println!("{:?}", tok);
        if tok.kind == TokenKind::EndOfFile {
            break;
        }
    }
    Ok(())
}
