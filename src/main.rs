extern crate impo;
use impo::cli;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = cli::parse_cli_args(&args);
    cli::run(&config);
}
