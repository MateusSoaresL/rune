// This 'enum' save the Token kinds.
#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // Keywords:
    NativePrint, // __print

    // Literals:
    LeftParen,  // (
    RightParen, // )

    // Others:
    Semicolon, // ;

    // Identifier and literal string:
    StringLiteral(String),

    // Specials:
    Error(String),   // For errors.
    Unknown(String), // For unknwon keywords.
    EOF,             // Represents the end of the file.
}

// This structure save some information's token.
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind, // Token kind.
    pub line: usize,     // Token line.
    pub column: usize,   // Token column.
}
