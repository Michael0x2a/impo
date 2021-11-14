use anyhow::Result;
use impo::{lexer::lex, parser::parse};

fn main() -> Result<()> {
    let tokens = lex("# test")?;
    let program = parse(&tokens)?;
    for stmt in program {
        println!("{:?}", stmt);
    }
    Ok(())
}
