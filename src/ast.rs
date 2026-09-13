// Math.
#[derive(Debug)]
pub enum BinaryOperator {
    Add,      // '+'.
    Subtract, // '-'.
    Multiply, // '*'.
    Divide,   // '/'.
}

// The expression.
#[derive(Debug)]
pub enum Expression {
    Identifier(String),    // Other keyword.
    StringLiteral(String), // "Hello, world!".
    NumberLiteral(f64),    // Number.

    // Math.
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
}

// The statements.
#[derive(Debug)]
pub enum Statement {
    // Print.
    Print {
        expression: Expression, // The expression.
        newline: bool,          // The line break.
    },

    // To print variable
    VariableDeclaration {
        name: String,
        value: Expression,
    },
}
