use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use impo::{lex, parse};
use impo::prettyprint::lisplike::prettyprint_program;
use anyhow::{Context, Result};
use pretty_assertions::assert_eq;

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

pub struct TestFile<'a> {
    pub name: &'a str,
    pub input: &'a str,
    pub expected: &'a str,
}

pub fn check_files(
    dir_path: impl AsRef<Path>,
    checker: fn(TestFile) -> Result<(), AnyError>,
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
        let (input, expected) = contents
            .split_once("---")
            .with_context(|| format!("Test file '{}' missing '---' delimiter", filename))?;

        checker(TestFile{
            name: &filename,
            input: input,
            expected: expected.trim_start(),
        })?;
    }

    Ok(())
}

pub fn project_relative_path(path: &'static str) -> PathBuf {
    let mut abs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    abs_path.push(Path::new(path));
    abs_path
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
    PrettyString(s.replace("\r\n", "\n").replace("\r", "\n"))
}

pub fn assert_str_eq(left: impl AsRef<str>, right: impl AsRef<str>) {
    pretty_assertions::assert_eq!(
        to_pretty_string(left.as_ref()),
        to_pretty_string(right.as_ref()),
    );
}