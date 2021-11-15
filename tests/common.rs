use std::error::Error;
use impo::{lex, parse};
use impo::prettyprint::lisplike::prettyprint_program;

pub type AnyError = Box<dyn Error>;

pub fn compile(text: impl AsRef<str>) -> Result<String, AnyError> {
    let tokens = lex(text.as_ref())?;
    let ast = parse(&tokens)?;
    Ok(prettyprint_program(&ast))
}

pub fn check(input: impl AsRef<str>, expected_output: impl AsRef<str>) -> Result<(), AnyError> {
    assert_eq!(compile(input)?, expected_output.as_ref());
    Ok(())
}