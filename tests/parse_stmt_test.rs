pub mod common;

use common::*;

#[test]
fn test_parsing_statements() -> Result<(), AnyError> {
    check_files(
        project_relative_path("./tests/parse_test_cases"),
        |test| {
            let outcome = compile(&test.input_source_code);
            match outcome {
                Ok(actual) => {
                    if let Some(ref expected_parse_tree) = test.expected_parse_tree {
                        assert_str_eq(actual, expected_parse_tree, &test.context);
                    }
                },
                Err(err) => {
                    if let Some(ref expected_err) = test.expected_output {
                        assert_str_eq(format!("{:?}", err), expected_err, &test.context);
                    } else {
                        return Err(err)
                    }
                }
            }
            
            Ok(())
        }
    )
}