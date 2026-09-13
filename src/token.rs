// TokenKind.
#[derive(Debug)]
pub enum TokenKind {
    Identifier(String),    // Other keyword.
    StringLiteral(String), // "Hello, world!".
    NumberLiteral(f64),    // Numbers.

    Plus,  // '+'.
    Minus, // '-'.
    Star,  // '*'.
    Slash, // '/'.

    LeftParen,  // '('.
    RightParen, // ')'.

    Semicolon, // ';'.

    EOF, // End Of File.
}

// Token.
pub struct Token {
    pub kind: TokenKind, // TokenKind.
    pub line: usize,     // Line.
    pub column: usize,   // Column.
}
