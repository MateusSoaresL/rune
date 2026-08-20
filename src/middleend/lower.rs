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
            },
        }
    }

    // Returns the Ir program.
    IrProgram { instructions }
}
