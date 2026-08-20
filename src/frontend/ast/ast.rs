// Expression.
#[derive(Debug)]
pub enum Expr {
    String(String), // The variable string.
}

// The statement.
#[derive(Debug)]
pub enum Statement {
    // The '__print("value");'.
    NativePrint(Expr),

    // The '__println("value");'.
    NativePrintln(Expr),
}

// The program.
#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>, // All statements.
}
