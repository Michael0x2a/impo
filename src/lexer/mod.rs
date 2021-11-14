mod core;
mod char_stream;
mod lex_numbers;
mod lex_simple;
mod lex_strings;

#[cfg(test)]
mod test_utils;

pub use self::core::lex;