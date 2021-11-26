use std::error::Error;
use impo::{lex, parse};
use impo::prettyprint::lisplike::prettyprint_program;
use anyhow::{Context, Result};

pub type AnyError = Box<dyn Error>;

pub fn compile(text: impl AsRef<str>) -> Result<String, AnyError> {
    let t = text.as_ref();
    let tokens = lex(t)
        .with_context(|| format!("Error lexing '{}'", t))?;
    let ast = parse(&tokens)
        .with_context(|| format!("Error parsing '{}'", t))?;
    Ok(prettyprint_program(&ast))
}

pub fn check(input: impl AsRef<str>, expected_output: impl AsRef<str>) -> Result<(), AnyError> {
    assert_eq!(compile(input)?, expected_output.as_ref());
    Ok(())
}