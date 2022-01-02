use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use impo::{lex, parse};
use impo::prettyprint::lisplike::prettyprint_program;
use anyhow::{Context, Result, anyhow};
use pretty_assertions::assert_eq;

use nom::branch::{
    alt,
};
use nom::bytes::complete::{
    tag,
    take_until,
    take_while_m_n,
};
use nom::character::complete::{
    line_ending,
    not_line_ending,
};
use nom::combinator::{
    complete,
    map,
    opt,
    value,
};
use nom::multi::{
    count,
    many0,
};
use nom::sequence::{
    delimited,
    pair,
    preceded,
    terminated,
    tuple,
};


pub type AnyError = anyhow::Error;

pub fn compile(text: impl AsRef<str>) -> Result<String, AnyError> {
    let tokens = lex(text)
        .context("Error lexing")?;
    let ast = parse(&tokens)
        .context("Error parsing")?;
    Ok(prettyprint_program(ast))
}

pub fn check(input: impl AsRef<str>, expected_output: impl AsRef<str>) -> Result<(), AnyError> {
    let input_str = input.as_ref();
    let actual = compile(input_str).with_context(|| format!("Checking '{}'", input_str))?;
    assert_eq!(actual, expected_output.as_ref());
    Ok(())
}

pub struct TestFile {
    pub context: String,

    pub input_source_code: String,

    pub expected_parse_tree: Option<String>,
    pub expected_error: Option<String>,
    pub expected_output: Option<String>,
}

pub fn check_files(
    dir_path: impl AsRef<Path>,
    checker: fn(&TestFile) -> Result<(), AnyError>,
) -> Result<(), AnyError> {
    let dir_path = dir_path.as_ref();
    let entries = fs::read_dir(&dir_path)
        .with_context(|| format!("Could not find '{}'", dir_path.display()))?;

    for maybe_entry in entries {
        let entry_path = maybe_entry
            .with_context(|| format!("Unexpected error scanning dir '{}'", dir_path.display()))?
            .path();

        let filename = entry_path
            .file_name()
            .context("No filename found")?
            .to_str()
            .context("Test file has non-utf-8 filename?")?;

        if !filename.ends_with(".test") {
            continue
        }

        let contents = fs::read_to_string(&entry_path)?;
        let tests = parse_test_file(filename, &contents)?;

        for test in &tests {
            checker(test).with_context(|| format!("Testing {}", test.context))?;
        }
    }

    Ok(())
}

pub fn project_relative_path(path: &'static str) -> PathBuf {
    let mut abs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    abs_path.push(Path::new(path));
    abs_path
}

fn parse_test_file<'a>(file_name: &str, contents: &'a str) -> Result<Vec<TestFile>, AnyError> {
    let parse_test_case = map(
        pair(
            terminated(
                take_until(":"),
                pair(tag(":"), line_ending),
            ),
            tuple((
                section_parser("code"),
                opt(section_parser("parse_tree")),
                opt(section_parser("output")),
                opt(section_parser("error")),
            )),
        ),
        move |(test_name, (code, parse_tree, output, error))| {
            TestFile {
                context: format!("'{}' :: {}", file_name, test_name),
                input_source_code: code,
                expected_parse_tree: parse_tree,
                expected_output: output,
                expected_error: error,
            }
        }
    );

    match complete(many0(parse_test_case))(contents) {
        Ok((rest, output)) => {
            if !rest.is_empty() {
                return Err(anyhow!("Could not parse test file '{}': parser did not match.\nRest = <<<{}>>>", file_name, rest))
            }
            return Ok(output)
        }
        Err(err) => {
            return Err(anyhow!("Could not parse test file '{}': {:?}", file_name, err))
        }
    }
}

fn section_parser<'a>(
    expected_name: &'static str
) -> impl FnMut(&'a str) -> nom::IResult<&'a str, String, nom::error::Error<&'a str>> 
{
    let parse_header_name = delimited(
        count(tag(" "), 4),
        tag(expected_name),
        tuple((tag(":"), line_ending)),
    );

    let parse_section_body = map(
        many0(
            map(
                pair(
                    alt((
                        preceded(
                            count(tag(" "), 8),
                            not_line_ending,
                        ),
                        value(
                            "",
                            take_while_m_n(0, 7, |c| c == ' '),
                        ),
                    )),
                    line_ending,
                ),
                |(text, newline)| format!("{}{}", text, newline),
            )
        ),
        |lines| { lines.join("") }
    );

    preceded(
        parse_header_name,        
        parse_section_body,
    )
}


// See https://github.com/colin-kiegel/rust-pretty-assertions/issues/24#issuecomment-520613247
#[derive(PartialEq, Eq)]
struct PrettyString(String);

/// Make diff to display string as multi-line string
impl fmt::Debug for PrettyString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

fn to_pretty_string(s: &str) -> PrettyString {
    PrettyString(s.trim().replace("\r\n", "\n").replace("\r", "\n"))
}

pub fn assert_str_eq(left: impl AsRef<str>, right: impl AsRef<str>, context: impl AsRef<str>) {
    pretty_assertions::assert_eq!(
        to_pretty_string(left.as_ref()),
        to_pretty_string(right.as_ref()),
        "Testing {}",
        context.as_ref(),
    );
}