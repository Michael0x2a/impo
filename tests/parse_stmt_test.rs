mod common;

use common::*;
use anyhow::Context;

#[test]
fn test_parsing_statements() -> Result<(), AnyError> {
    check_files(
        project_relative_path("./tests/parse_test_cases"),
        |test| {
            let actual = compile(test.input).with_context(|| format!("Checking '{}'", test.name))?;
            assert_str_eq(actual, test.expected);
            Ok(())
        }
    )
}