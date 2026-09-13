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
    
    // Just peek.
    fn peek(&self) -> Option<char> {
        self.source.get(self.position + 1).copied()
    }

    // For read the number.
    fn read_number(&mut self) -> Result<f64, String> {
        let start = self.position;

        // Integer part.
        while let Some(character) = self.position() {
            if character.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        // Decimal part.
        if self.position() == Some('.')
            && self.peek().is_some_and(|character| character.is_ascii_digit())
            {
                self.advance(); // Consume '.'.

                while let Some(character) = self.position() {
                    if character.is_ascii_digit() {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }

        let number: String = self.source[start..self.position].iter().collect();

        number
            .parse::<f64>()
            .map_err(|_| format!("Invalid number '{}'", number))
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

                // Plus.
                '+' => {
                    tokens.push(Token {
                        kind: TokenKind::Plus,
                        line: self.line,
                        column: self.column,
                    });

                    self.advance();
                }

                // Minus.
                '-' => {
                    tokens.push(Token {
                        kind: TokenKind::Minus,
                        line: self.line,
                        column: self.column,
                    });

                    self.advance();
                }

                // Star.
                '*' => {
                    tokens.push(Token {
                        kind: TokenKind::Star,
                        line: self.line,
                        column: self.column,
                    });

                    self.advance();
                }

                // Slash.
                '/' => {
                    tokens.push(Token {
                        kind: TokenKind::Slash,
                        line: self.line,
                        column: self.column,
                    });

                    self.advance();
                }

                // Equal.
                '=' => {
                    tokens.push(Token {
                        kind: TokenKind::Equal,
                        line: self.line,
                        column: self.column,
                    });

                    self.advance();
                }

                // Numbers.
                character if character.is_ascii_digit() || character == '.' => {
                    // Before save the line and column.
                    let line = self.line;
                    let column = self.column;

                    let value = self.read_number()?;

                    tokens.push(Token {
                        kind: TokenKind::NumberLiteral(value),
                        line,
                        column,
                    });
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
