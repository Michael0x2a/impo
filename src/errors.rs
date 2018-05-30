use std::convert::From;
use text::{TextRange, LinesWithEndings};

#[derive(Fail, Debug, Clone, PartialEq, Eq, Hash)]
#[fail(display="{:?}: {}", range, description)]
pub struct ImpoError {
    pub range: TextRange,
    pub error_stage: ErrorStage,
    pub description: String,
}

impl ImpoError {
    pub fn new(range: TextRange, error_stage: ErrorStage, description: &str) -> ImpoError {
        ImpoError { range, error_stage, description: description.to_owned() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ErrorStage {
    Tokenizing,
    Parsing,
    Typechecking,
    Interpreting,
}

#[derive(Fail, Debug)]
#[fail(display="Multiple errors (use .errors field for full list)")]
pub struct ErrorGroup {
    pub errors: Vec<ImpoError>,
}

impl ErrorGroup {
    pub fn wrap(error: ImpoError) -> ErrorGroup {
        ErrorGroup { errors: vec![error] }
    }

    pub fn new(mut errors: Vec<ImpoError>) -> ErrorGroup {
        errors.sort_by_key(|e| e.range);
        ErrorGroup { errors }
    }

    pub fn format_against(&self, text: &str) -> Vec<String> {
        let mut char_count = 0;
        let mut output = Vec::new();
        let mut counter: usize = 0;
        for (idx, line) in LinesWithEndings::from(text).enumerate() {
            let lineno = idx + 1;
            let prev_char_count = char_count;
            char_count += line.len();
            while counter < self.errors.len() {
                let err = &self.errors[counter];
                if err.range.offset() < char_count {
                    let column = err.range.offset() - prev_char_count;
                    counter += 1;
                    output.push(format!(
                        "Error on line {}, column {}:\n\n  {}  {}{}\n\n  {}\n",
                        lineno,
                        column,
                        line,
                        " ".repeat(column),
                        "^".repeat(err.range.length()),
                        &err.description,
                    ))
                } else {
                    break;
                }
            }
        }
        output
    }
}

impl From<ImpoError> for ErrorGroup {
    fn from(err: ImpoError) -> ErrorGroup {
        ErrorGroup::wrap(err)
    }
}
