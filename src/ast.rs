// The expression.
#[derive(Debug)]
pub enum Expression {
    Identifier(String),    // Other keyword.
    StringLiteral(String), // "Hello, world!"
}

// The statements.
#[derive(Debug)]
pub enum Statement {
    // Print.
    Print {
        expression: Expression, // The expression.
        newline: bool,          // The line break.
    },
}
