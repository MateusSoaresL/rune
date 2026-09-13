use crate::ast::{BinaryOperator, Expression, Statement};

pub enum Value {
    String(String),
    Number(f64),
}

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    // For run the StringLiteral and Identifier.
    fn evaluate_expression(&self, expression: &Expression) -> Result<Value, String> {
        match expression {
            Expression::StringLiteral(value) => {
                Ok(Value::String(value.clone()))
            }

            Expression::NumberLiteral(value) => {
                Ok(Value::Number(value.clone()))
            }

            Expression::Identifier(name) => {
                Err(format!(
                    "Undefined variable '{}'",
                    name
                ))
            }

            Expression::Binary { left, operator, right } => {
                let left = self.evaluate_expression(left)?;
                let right = self.evaluate_expression(right)?;

                match (left, right) {
                    (
                        Value::Number(left),
                        Value::Number(right),
                    ) => {
                        let result = match operator {
                            BinaryOperator::Add => {
                                left + right
                            }

                            BinaryOperator::Subtract => {
                                left - right
                            }

                            BinaryOperator::Multiply => {
                                left * right
                            }

                            BinaryOperator::Divide => {
                                if right == 0.0 {
                                    return Err(
                                        "Division by zero".to_string()
                                    );
                                }

                                left / right
                            }
                        };

                        Ok(Value::Number(result))
                    }

                    _ => Err(
                        "Arithmetic operators require numbers".to_string()
                    ),
                }
            }
        }
    }

    // For execute the statements
    fn execute_statement(&self, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::Print {
                expression,
                newline,
            } => {
                let value = self.evaluate_expression(expression)?;

                let output = match value {
                    Value::String(value) => value,
                    Value::Number(value) => value.to_string()
                };

                if *newline {
                    println!("{}", output);
                } else {
                    print!("{}", output);
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
