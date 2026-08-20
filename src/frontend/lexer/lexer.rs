use crate::frontend::lexer::token::{Token, TokenKind};

// Is the principal lexer structure, and save some information's lexer.
pub struct Lexer {
    source: Vec<char>, // Is the code.
    current: usize,    // Is the current index.

    line: usize,   // Is the current line.
    column: usize, // Is the current column.
}

impl Lexer {
    // Create instance.
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(), // Save the code.
            current: 0,                       // The current index.

            line: 1,   // The line start.
            column: 1, // The column start.
        }
    }

    // Check if is at end.
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // Advance the character.
    fn advance(&mut self) -> char {
        let character = self.source[self.current]; // The current index.
        self.current += 1; // Advance the index,

        // A structure for this.
        if character == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        character // Returns the character.
    }

    // This function will check the '"' in the code.
    fn string(&mut self, line: usize, column: usize) -> Token {
        // Here is the value in '__print("value");'.
        let mut value = String::new();

        // This code will check if is at end.
        while !self.is_at_end() {
            let character = self.source[self.current]; // Here is the '"'.

            // If is the '"'.
            if character == '"' {
                self.advance(); // Consume the '"'.

                // And return the string literal '"'.
                return Token {
                    kind: TokenKind::StringLiteral(value), // Return the '"'.
                    line,                                  // Return the line.
                    column,                                // Return the column.
                };
            }

            // If dound '\', can be this sequence:
            if character == '\\' {
                self.advance(); // Consume the '\'.

                // Avoid try enter after the end of file.
                if self.is_at_end() {
                    break;
                }

                // Take the character after the '\'.
                let escape = self.advance();

                match escape {
                    // '\n' = line break.
                    'n' => value.push('\n'),

                    // '\t' = tabulation.
                    't' => value.push('\t'),

                    // '\"' = quotes inside the string.
                    '"' => value.push('"'),

                    // '\\' inverted bar.
                    '\\' => value.push('\\'),

                    // Unknwon escape.
                    _ => {
                        value.push('\\');
                        value.push(escape);
                    }
                }

                // Not execute the 'value.push()' down there again.
                continue;
            }

            // Normal character
            value.push(self.advance()); // Add in the value and advance.
        }

        // If the '"' do not close.
        eprintln!("String not finished in {}:{}", line, column);

        // Warning:
        // For the function do not return error in this part.
        return Token {
            kind: TokenKind::Error("Unterminated string".to_string()),
            line,
            column,
        };
    }

    // It servers to read a whole word.
    fn word(&mut self, first: char, line: usize, column: usize) -> Token {
        let mut word = String::new(); // Create a space to save the words.

        word.push(first); // Save the word.

        // Check if is at end,
        while !self.is_at_end() {
            let character = self.source[self.current]; // Is the character.

            // Check if is a letter or a number.
            if character.is_alphanumeric() || character == '_' {
                word.push(self.advance()); // Save and advance.
            } else {
                break; // Break if is not.
            }
        }

        // Check the token kind, example: '__print'.
        let kind = match word.as_str() {
            "__print" => TokenKind::NativePrint, // Return the 'NativePrint'.

            // Return error.
            _ => {
                eprintln!("Unknown word '{}' in {}:{}", word, line, column);
                TokenKind::Unknown(word)
            }
        };

        Token { kind, line, column } // Return token.
    }

    // Is the principal lexer controller.
    pub fn tokenize(&mut self) -> Vec<Token> {
        // Create a space to save the tokens.
        let mut tokens = Vec::new();

        // Check if is at end.
        while !self.is_at_end() {
            // Create a nickname for 'self.line'.
            let line = self.line;

            // Create a nickname for 'self.column'.
            let column = self.column;

            // The character to advance.
            let character = self.advance();

            // Check what character's kind is.
            match character {
                // If is '(', returns 'LeftParen'.
                '(' => {
                    tokens.push(Token {
                        kind: TokenKind::LeftParen,
                        line,
                        column,
                    });
                }

                // If is ')', returns 'RightParen'.
                ')' => {
                    tokens.push(Token {
                        kind: TokenKind::RightParen,
                        line,
                        column,
                    });
                }

                // If is ';', returns 'Semicolon'.
                ';' => {
                    tokens.push(Token {
                        kind: TokenKind::Semicolon,
                        line,
                        column,
                    });
                }

                // If is '"', returns a function.
                // and this function has 'StringLiteral'.
                '"' => {
                    tokens.push(self.string(line, column));
                }

                // If is a letter, save.
                character if character.is_alphabetic() || character == '_' => {
                    tokens.push(self.word(character, line, column));
                }

                // If is a whitespace, do nothing.
                character if character.is_whitespace() => {}

                // Returns an error.
                _ => {
                    eprintln!("Invalid character '{}' in {}:{}", character, line, column);
                    TokenKind::Unknown(character.to_string());
                }
            }
        }

        // Save the token EOF.
        tokens.push(Token {
            kind: TokenKind::EOF,
            line: self.line,
            column: self.column,
        });

        tokens // Returns token.
    }
}
