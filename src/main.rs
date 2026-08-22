use std::{env, fs, path::Path, process::Command};

use crate::{
    backend::cranelift::compile,
    frontend::{lexer::lexer::Lexer, parser::parser::Parser, semantic::semantic::SemanticAnalyzer},
    middleend::lower::lower,
};

mod backend;
mod frontend;
mod middleend;

fn main() {
    // Collect the arguments in the terminal.
    let args: Vec<String> = env::args().collect();

    // This code will check if the arguments in terminal is just:
    // './runec' or 'runec'.
    if args.len() < 3 {
        eprintln!("Help: using <runec> <file.rune>");
        std::process::exit(1); // Will stop the program.
    }

    // Rune file:
    //
    // Example:
    // main.rune
    let file = &args[1];

    // Command:
    //
    // "run".
    // Or:
    // "build".
    let command = &args[2];

    // Create a way's representation.
    let path = Path::new(file);

    // Verifiy if the file finish in .rune
    if path.extension().and_then(|extension| extension.to_str()) != Some("rune") {
        eprintln!("Error: souce file must use the '.rune' extension.");
        std::process::exit(1);
    }

    // Read the .rune file whole.
    let source = match fs::read_to_string(file) {
        Ok(source) => source,

        Err(error) => {
            eprintln!("Error reading '{}': {}", file, error);

            std::process::exit(1);
        }
    };

    // ==========|
    // FRONT-END |
    // ==========|

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(program) => program,

        Err(_) => {
            std::process::exit(1);
        }
    };
    let mut semantic = SemanticAnalyzer::new();
    if semantic.analyze(&ast).is_err() {
        std::process::exit(1);
    }

    // ===========|
    // MIDDLE-END |
    // ===========|

    let ir = lower(ast);

    // =========|
    // BACK-END |
    // =========|

    if let Err(error) = compile(&ir) {
        eprintln!("Backend error: {}", error);
        std::process::exit(1);
    }

    // ============|
    // RUNTIME-END |
    // ============|

    // Mount:
    //
    // src/runtime/runtime.asm -> src/runtime/runtime.o
    let nasm_status = match Command::new("nasm")
        .args([
            "-f",
            "elf64",
            "src/runtime/runtime.asm",
            "-o",
            "src/runtime/runtime.o",
        ])
        .status()
    {
        Ok(status) => status,

        Err(error) => {
            eprintln!("Could not execute NASM: {}", error);
            std::process::exit(1);
        }
    };

    // NASM executeed, but could found an error in assembly.
    if !nasm_status.success() {
        eprintln!("Failed to compile Rune runtime!");
        std::process::exit(1);
    }

    // ======|
    // LINKER|
    // ======|

    // Mount:
    //
    // src/runtime/runtime.o
    //          +
    // src/runtime/program.o
    //
    // And generates:
    //
    // program
    let linker_status = match Command::new("ld")
        .args([
            "src/runtime/runtime.o",
            "src/runtime/program.o",
            "-o",
            "program",
        ])
        .status()
    {
        Ok(status) => status,

        Err(error) => {
            eprintln!("Could not execute linker: {}", error);
            std::process::exit(1);
        }
    };

    if !linker_status.success() {
        eprintln!("Linking failed!");
        std::process::exit(1);
    }

    // ========|
    // COMMAND |
    // ========|

    match command.as_str() {
        // Just compile.
        "build" => {
            println!("Generated './program'");
        }

        // Compile and execute.
        "run" => {
            let status = Command::new("./program").status();

            match status {
                Ok(status) => {
                    if !status.success() {
                        eprintln!("Program exited with status: {}", status);
                    }
                }

                Err(error) => {
                    eprintln!("Could not execute program: {}", error);
                    std::process::exit(1);
                }
            }
        }

        _ => {
            eprintln!("Unknown command '{}'! Use 'run' or 'build'!", command);
            std::process::exit(1);
        }
    }
}
