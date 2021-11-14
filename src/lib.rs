#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]

#![allow(clippy::redundant_field_names)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::missing_errors_doc)]

// Items from clippy::restriction
#![warn(clippy::as_conversions)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::create_dir)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::default_numeric_fallback)]
#![warn(clippy::exit)]
#![warn(clippy::expect_used)]
#![warn(clippy::filetype_is_file)]
#![warn(clippy::fn_to_numeric_cast)]
#![warn(clippy::get_unwrap)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::let_underscore_must_use)]
#![warn(clippy::lossy_float_literal)]
#![warn(clippy::map_err_ignore)]
#![warn(clippy::mem_forget)]
#![warn(clippy::multiple_inherent_impl)]
#![warn(clippy::panic)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]
#![warn(clippy::rc_buffer)]
#![warn(clippy::rc_mutex)]
#![warn(clippy::rest_pat_in_fully_bound_structs)]
//#![warn(clippy::same_name_method)]
#![warn(clippy::str_to_string)]
//#![warn(clippy::string_slice)]
#![warn(clippy::string_to_string)]
#![warn(clippy::todo)]
//#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::unimplemented)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(clippy::unreachable)]
#![warn(clippy::unwrap_in_result)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::verbose_file_reads)]


pub mod values;
pub mod tokens;
pub mod lexer;
pub mod ast;
pub mod parser;
//pub mod sorted_lookup;
