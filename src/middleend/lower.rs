use crate::{
    frontend::ast::ast::{Expr, Program, Statement},
    middleend::ir::{IrInstructions, IrProgram},
};

// This function take the AST and transforms in Rune IR.
pub fn lower(program: Program) -> IrProgram {
    // Represents the instruction in the program.
    let mut instructions = Vec::new();

    // Will do a loop for the statements.
    for statement in program.statements {
        // Will check the tokens.
        match statement {
            // '__print("value");'.
            Statement::NativePrint(expr) => match expr {
                // Will add the statement in the instructions.
                Expr::String(value) => {
                    instructions.push(IrInstructions::PrintString(value));
                }

                Expr::Variable(name) => {
                    instructions.push(IrInstructions::PrintVariable(name));
                }
            },

            // '__println("value");'.
            Statement::NativePrintln(expr) => match expr {
                // Will add the statement in the instructions.
                Expr::String(value) => {
                    instructions.push(IrInstructions::PrintlnString(value));
                }

                Expr::Variable(name) => {
                    instructions.push(IrInstructions::PrintlnVariable(name));
                }
            },

            Statement::VariableDeclaration {
                name,
                value,
                mutable,
            } => match value {
                Expr::String(value) => {
                    instructions.push(IrInstructions::DeclareVariable {
                        name,
                        value,
                        mutable,
                    });
                }

                _ => {
                    eprintln!("Lowering error: unsupported value in variable '{}'!", name,);

                    std::process::exit(1);
                }
            },

            Statement::Assignment { name, value } => match value {
                Expr::String(value) => {
                    instructions.push(IrInstructions::AssignVariable { name, value });
                }

                _ => {
                    eprintln!(
                        "Lowering error: unsupported assignment value for '{}'!",
                        name
                    );

                    std::process::exit(1);
                }
            },
        }
    }

    // Returns the Ir program.
    IrProgram { instructions }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::ast::{Expr, Program, Statement};

    #[test]
    fn test_lower_prints() {
        let ast = Program {
            statements: vec![
                Statement::NativePrint(Expr::String("Hello, ".to_string())),
                Statement::NativePrintln(Expr::String("world!".to_string())),
            ],
        };

        let ir = lower(ast);

        println!("{:#?}", ir);
    }
}
