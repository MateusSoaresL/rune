use std::collections::HashMap;

use crate::frontend::ast::ast::{Expr, Program, Statement, StringPart};

// Create a data, if the variable is mutable or not.
pub struct Symbol {
    pub mutable: bool,
}

// The principal Semantic Analyzer structure.
pub struct SemanticAnalyzer {
    symbols: HashMap<String, Symbol>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    // Analyze the variable expression.
    fn analyze_expression(&self, expression: &Expr) -> Result<(), ()> {
        // Check the expression.
        match expression {
            Expr::String(_) => Ok(()),

            // This part check this:
            //
            // let name = "Rune";
            // __println(name);
            //
            // Pass.
            //
            // But this:
            //
            // __println(name);
            //
            // Do not pass.
            Expr::Variable(name) => {
                if !self.symbols.contains_key(name) {
                    eprintln!("Undefined variable '{}'!", name);

                    return Err(());
                }

                Ok(())
            }

            Expr::InterpolatedString(parts) => {
                for part in parts {
                    match part {
                        StringPart::Text(_) => {}

                        StringPart::Variable(name) => {
                            if !self.symbols.contains_key(name) {
                                eprintln!(
                                    "Undefined variable '{}' in string in interpolation",
                                    name
                                );

                                return Err(());
                            }
                        }
                    }
                }

                Ok(())
            }
        }
    }

    // Analyze the statements.
    fn analyze_statement(&mut self, statement: &Statement) -> Result<(), ()> {
        match statement {
            Statement::Assignment { name, value } => {
                // First, search the variable.
                let symbol = match self.symbols.get(name) {
                    Some(symbol) => symbol,

                    None => {
                        eprintln!("Undefined variable '{}'!", name);

                        return Err(());
                    }
                };

                // Now, verify if they is mutable.
                if !symbol.mutable {
                    eprintln!("Cannot assign to immutable variable '{}'!", name);

                    return Err(());
                }

                // Verify with semantic the new value.
                self.analyze_expression(value)?;

                // Returns.
                Ok(())
            }

            Statement::VariableDeclaration {
                name,
                value,
                mutable,
            } => {
                // This part is this:
                //
                // let name = "Rune";
                // let name = "Other";
                //
                // And give error.
                if self.symbols.contains_key(name) {
                    eprintln!("Variable '{}' is already declared!", name);

                    return Err(());
                }

                // Use the analyze expression.
                self.analyze_expression(value)?;

                // For the mutable.
                self.symbols
                    .insert(name.clone(), Symbol { mutable: *mutable });

                // Returns.
                Ok(())
            }

            // For '__print' and '__println'.
            Statement::NativePrint(expr) | Statement::NativePrintln(expr) => {
                self.analyze_expression(expr)
            }
        }
    }

    // This function through the AST.
    pub fn analyze(&mut self, program: &Program) -> Result<(), ()> {
        // Through the AST.
        for statement in &program.statements {
            self.analyze_statement(statement)?;
        }

        // Returns.
        Ok(())
    }
}
