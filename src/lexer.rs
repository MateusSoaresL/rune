use crate::token::{Token, TokenKind};

pub struct Lexer {
    source: Vec<char>, // The source.
    position: usize,   // The lexer position.
    line: usize,       // The lexer line.
    column: usize,     // The lexer column.
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    // The lexer position.
    pub fn position(&self) -> Option<char> {
        self.source.get(self.position).copied()
    }

    // The lexer advance.
    fn advance(&mut self) -> Option<char> {
        let character = self.position()?;

        self.position += 1;

        if character == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        Some(character)
    }

    // This function read the identifier and returns a string.
    fn read_identifier(&mut self) -> String {
        let start = self.position;

        while let Some(character) = self.position() {
            if character.is_ascii_alphanumeric() {
                self.advance();
            } else {
                break;
            }
        }

        // Take the Vec<char> part and returns a string.
        self.source[start..self.position].iter().collect()
    }

    // For read the string "".
    fn read_string(&mut self) -> Result<String, String> {
        self.advance();

        let start = self.position;

        while let Some(character) = self.position() {
            if character == '"' {
                let value = self.source[start..self.position].iter().collect();

                self.advance();

                return Ok(value);
            }

            self.advance();
        }

        Err(format!(
            "Unterminated string at {}:{}",
            self.line, self.column
        ))
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(character) = self.position() {
            match character {
                // Ignore whitespace.
                ' ' | '\t' | '\r' => {
                    self.advance();
                }

                // Ignore new line.
                '\n' => {
                    self.advance();
                }

                '"' => {
                    let line = self.line;
                    let column = self.column;

                    let value = self.read_string()?;

                    tokens.push(Token {
                        kind: TokenKind::StringLiteral(value),
                        line,
                        column,
                    });
                }

                // LeftParen.
                '(' => {
                    tokens.push(Token {
                        kind: TokenKind::LeftParen,
                        line: self.line,
                        column: self.column,
                    });

                    self.advance();
                }

                // RightParen.
                ')' => {
                    tokens.push(Token {
                        kind: TokenKind::RightParen,
                        line: self.line,
                        column: self.column,
                    });

                    self.advance();
                }

                // Semicolon.
                ';' => {
                    tokens.push(Token {
                        kind: TokenKind::Semicolon,
                        line: self.line,
                        column: self.column,
                    });

                    self.advance();
                }

                // Check the keywords.
                character if character.is_ascii_alphabetic() => {
                    // Before save the line and column.
                    let line = self.line;
                    let column = self.column;

                    let identifier = self.read_identifier();

                    let kind = match identifier.as_str() {
                        "print" => TokenKind::Identifier(identifier),
                        "println" => TokenKind::Identifier(identifier),

                        _ => TokenKind::Identifier(identifier),
                    };

                    tokens.push(Token { kind, line, column });
                }

                _ => {
                    return Err(format!(
                        "Unexpected character '{}' at {}:{}",
                        character, self.line, self.column
                    ));
                }
            }
        }

        tokens.push(Token {
            kind: TokenKind::EOF,
            line: self.line,
            column: self.column,
        });

        Ok(tokens)
    }
}
