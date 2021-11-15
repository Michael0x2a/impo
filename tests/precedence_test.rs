mod common;
use common::*;

#[test]
fn test_basic_operators() -> Result<(), AnyError> {
    check(
        "a + b * c",
        "(infix a + (infix b * c))",
    )?;
    check(
        "a + b - c * d / e",
        "(infix a + b - (infix c * d / e))",
    )?;
    check(
        "(((a)))",
        "(paren (paren (paren a)))",
    )?;
    check(
        "a and b or c and d or e",
        "(infix (infix a and b) or (infix c and d) or e)",
    )?;

    Ok(())
}