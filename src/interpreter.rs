use crate::ast::{Expression, Statement};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    // For run the StringLiteral and Identifier.
    fn evaluate_expression(&self, expression: &Expression) -> Result<String, String> {
        match expression {
            Expression::StringLiteral(value) => {
                Ok(value.clone())
            }

            Expression::Identifier(name) => {
                Err(format!(
                    "Undefined variable '{}'",
                    name
                ))
            }
        }
    }

    // For execute the statements
    fn execute_statement(&self, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::Print { expression, newline } => {
                let value = self.evaluate_expression(expression)?;

                if *newline {
                    println!("{}", value);
                } else {
                    print!("{}", value);
                }

                Ok(())
            }
        }
    }

    // Run the program
    pub fn run(&self, statements: &[Statement]) -> Result<(), String> {
        for statement in statements {

            self.execute_statement(statement)?;
        }

        Ok(())
    }
}
