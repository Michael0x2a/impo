use anyhow::Result;
use impo::{lex, parse};

fn main() -> Result<()> {
    let tokens = lex("# test")?;
    let program = parse(&tokens)?;
    for stmt in &program.body {
        println!("{:?}", stmt);
    }
    Ok(())
}
