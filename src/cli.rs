use runner;

use std::process;
use std::path::{Path, PathBuf};

pub enum CliCommand {
    PrintHelp(i32),
    PrintVersion,
    InterpretFile(PathBuf),
    RunRepl,
}

pub fn run(config: &CliCommand) {
    match config {
        &CliCommand::PrintHelp(error_code) => {
            print_usage_and_die(error_code);
        },
        &CliCommand::PrintVersion => {
            println!("Version 0.0.1");
        },
        &CliCommand::InterpretFile(ref path) => {
            runner::run_file(path.as_path());
        },
        &CliCommand::RunRepl => {
            runner::run_repl();
        }
    }
}

pub fn parse_cli_args(args: &[String]) -> CliCommand {
    if args.len() == 1 {
        return CliCommand::PrintHelp(1);
    }
    let first_arg = &args[1];
    if first_arg == "--help" || first_arg == "-h" {
        return CliCommand::PrintHelp(0);
    }
    if first_arg == "--version" || first_arg == "v" {
        return CliCommand::PrintVersion;
    }
    if first_arg == "repl" {
        return CliCommand::RunRepl;
    }
    if first_arg == "run" && args.len() == 2 {
        let second_arg = &args[2];
        let path = Path::new(second_arg);
        if path.is_file() {
            return CliCommand::InterpretFile(path.to_path_buf());
        }
    }
    return CliCommand::PrintHelp(2);
}

pub fn print_usage() {
    println!("Impo -- a toy imperative, object-oriented language");
    println!();
    println!("Usage:");
    println!();
    println!("  impo run FILE   Interprets the given file");
    println!("  impo repl       Starts the REPL");
    println!();
}

pub fn print_usage_and_die(error_code: i32) -> ! {
    print_usage();
    process::exit(error_code);
}
