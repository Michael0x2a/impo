use std::path::Path;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use parser::scanner::TokenStream;
use parser::parser::Parser;
use typecheck::infer::InferenceEngine;
use interpreter::interpreter::Interpreter;
use ast::stringpool::StringPool;
use ast::untyped::UntypedNodeVisitor;
use ast::typed::{MutatingTypedNodeVisitor, TypedNodeVisitor, TypedNodeToStrVisitor};
use errors::ErrorGroup;

static DEBUG: bool = false;

pub fn run(contents: &str) -> Result<(), ErrorGroup> {
    // Tokenize
    let mut tokens = Vec::new();
    TokenStream::new(contents.chars(), &mut tokens).scan()?;
    if DEBUG {
        for token in &tokens {
            println!("{:?}", token);
        }
    }

    // Parse
    let mut pool = StringPool::new();
    let untyped_ast = Parser::new(tokens.iter(), &mut pool).parse_as_line()?;

    // Infer
    let mut inference = InferenceEngine::new();
    let typed_ast = inference.visit_expr(&untyped_ast);
    if !inference.errors.is_empty() {
        return Err(ErrorGroup::new(inference.errors));
    }
    if DEBUG {
        println!("{:?}", TypedNodeToStrVisitor::new(&pool).visit_expr(&typed_ast));
    }

    // Interpreter
    let mut interpreter = Interpreter::new(&mut pool);
    let val = interpreter.visit_expr(&typed_ast)?;
    println!("{}", interpreter.display(&val));
    Ok(())
}

pub fn run_file(path: &Path) {
    let mut f = File::open(path).expect("File not found!");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Error while reading file");
    if let Err(e) = run(&contents) {
        for err in e.format_against(&contents) {
            println!("{}", err);
        }
    }
}

pub fn run_repl() {
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    loop {
        print!("impo: ");
        stdout.flush().unwrap();

        let mut user_input = String::new();
        stdin.read_line(&mut user_input).expect("Error reading from stdin");

        if let Err(e) = run(&user_input) {
            for err in e.format_against(&user_input) {
                println!("{}", err);
            }
        }
        println!();
    }
}