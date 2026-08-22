// Expression.
#[derive(Debug)]
pub enum Expr {
    String(String),   // The variable string.
    Variable(String), // The variable.
}

// The statement.
#[derive(Debug)]
pub enum Statement {
    // The '__print("value");'.
    NativePrint(Expr),

    // The '__println("value");'.
    NativePrintln(Expr),

    // For the variables.
    VariableDeclaration {
        name: String,  // The name.
        value: Expr,   // The value.
        mutable: bool, // If is mutable or not.
    },

    // For mutable variable.
    Assignment {
        name: String, // The name.
        value: Expr,  // The new value.
    },
}

// The program.
#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>, // All statements.
}
