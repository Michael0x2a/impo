pub mod common;
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
    check(
        "- a - - - b - c",
        "(infix (- a) - (- (- b)) - c)",
    )?;
    check(
        "a < b and c == d or -e > f + g",
        "(infix (infix (infix a < b) and (infix c == d)) or (infix (- e) > (infix f + g)))",
    )?;
    check(
        "~a | ~b & ~c ^ ~d << ~e",
        "(infix (~ a) | (infix (infix (~ b) & (~ c)) ^ (infix (~ d) << (~ e))))",
    )?;
    check(
        "a to b and c to d",
        "(infix (range a b) and (range c d))",
    )?;
    check(
        "a to b == c to d == e to f",
        "(infix (range a b) == (range c d) == (range e f))",
    )?;

    Ok(())
}

#[test]
fn test_lookup() -> Result<(), AnyError> {
    check(
        "a.b",
        "(lookup a b)",
    )?;
    check(
        "3.b",
        "(lookup 3 b)",
    )?;
    check(
        "(a.b).c",
        "(lookup (paren (lookup a b)) c)",
    )?;
    check(
        "tup.1",
        "(lookup tup 1)",
    )?;
    // TODO: Fix me, after moving generating floats up to the parsing stage
    /*check(
        "tup.1.2.3",
        "(lookup tup 1 2 3)",
    )?;*/

    Ok(())
}

#[test]
fn test_func_calls() -> Result<(), AnyError> {
    check(
        "a()",
        "(call a)",
    )?;
    check(
        "a(p0)",
        "(call a p0)",
    )?;
    check(
        "a(p0, p1, p2)",
        "(call a p0 p1 p2)",
    )?;
    check(
        "a.b.c(p0, p1, p2)",
        "(call (lookup a b c) p0 p1 p2)",
    )?;
    check(
        "f()()()",
        "(call (call (call f)))",
    )?;
    check(
        "f(a)(b)(c)",
        "(call (call (call f a) b) c)",
    )?;
    check(
        "(a.b).c.f()((a))()()",
        "(call (call (call (call (lookup (paren (lookup a b)) c f)) (paren a))))",
    )?;

    Ok(())
}

#[test]
fn test_index() -> Result<(), AnyError> {
    check(
        "a[b]",
        "(index a b)",
    )?;
    check(
        "a[b][c][d]",
        "(index (index (index a b) c) d)",
    )?;
    check(
        "a[b](c)[d](e)",
        "(call (index (call (index a b) c) d) e)",
    )?;
    check(
        "a[x to y]",
        "(index a (range x y))",
    )?;
    check(
        "a[1 + x to 3 * y]",
        "(index a (range (infix 1 + x) (infix 3 * y)))",
    )?;

    Ok(())
}