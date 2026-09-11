use std::{env, fs, path::Path};

use crate::{interpreter::Interpreter, lexer::Lexer, parser::Parser};

mod ast;
mod interpreter;
mod lexer;
mod parser;
mod token;

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Usage: rune <file.rune>".to_string());
    }

    let path = &args[1];

    let extension = Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str());

    if extension != Some("rune") {
        return Err(format!(
            "'{}' is not a .rune file",
            path
        ));
    }

    let source = fs::read_to_string(path)
        .map_err(|error| error.to_string())?;

    let mut lexer = Lexer::new(source);

    let tokens = lexer.tokenize()?;

    let mut parser = Parser::new(tokens);

    let statements = parser.parse()?;

    let interpreter = Interpreter::new();

    interpreter.run(&statements)?;

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}
