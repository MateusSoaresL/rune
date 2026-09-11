use crate::{
    ast::{Expression, Statement},
    token::{Token, TokenKind},
};

pub struct Parser {
    tokens: Vec<Token>, // The token vector.
    position: usize,    // The parser position.
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    // The lexer position.
    fn position(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    // The lexer advance.
    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    // The parser expression.
    fn parse_expression(&mut self) -> Result<Expression, String> {
        let expression = match self.position() {
            Some(Token {
                kind: TokenKind::StringLiteral(value),
                ..
            }) => Expression::StringLiteral(value.clone()),

            Some(Token {
                kind: TokenKind::Identifier(name),
                ..
            }) => Expression::Identifier(name.clone()),

            Some(Token {
                kind: TokenKind::EOF,
                line,
                column,
            }) => {
                return Err(format!(
                    "Expected expression, found end of file at {}:{}",
                    line, column
                ));
            }

            Some(token) => {
                return Err(format!(
                    "Expected expression, found {:?} at {}:{}",
                    token.kind, token.line, token.column
                ));
            }

            None => {
                return Err("Unexpected parser state: missing EOF token".to_string());
            }
        };

        self.advance();

        Ok(expression)
    }

    // The expect function.
    fn expect(&mut self, expected: TokenKind) -> Result<(), String> {
        let token = self
            .position()
            .ok_or("Unexpected end of file".to_string())?;

        if std::mem::discriminant(&token.kind) == std::mem::discriminant(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Expected {:?}, found {:?} at {}:{}",
                expected, token.kind, token.line, token.column
            ))
        }
    }

    // The parser print.
    fn parse_print(&mut self, newline: bool) -> Result<Statement, String> {
        self.expect(TokenKind::LeftParen)?;

        let expression = self.parse_expression()?;

        self.expect(TokenKind::RightParen)?;
        self.expect(TokenKind::Semicolon)?;

        Ok(Statement::Print {
            expression,
            newline,
        })
    }

    // The parser statements.
    fn parse_statement(&mut self) -> Result<Statement, String> {
        let name = match self.position() {
            Some(Token {
                kind: TokenKind::Identifier(name),
                ..
            }) => name.clone(),

            _ => {
                return Err("Expected statement".to_string());
            }
        };

        match name.as_str() {
            "print" => {
                self.advance();
                self.parse_print(false)
            }

            "println" => {
                self.advance();
                self.parse_print(true)
            }

            _ => Err(format!("Unknown statement '{}'", name)),
        }
    }

    // Check if is at end.
    fn is_at_end(&self) -> bool {
        matches!(
            self.position(),
            Some(Token {
                kind: TokenKind::EOF,
                ..
            })
        )
    }

    // The principal parser function.
    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        Ok(statements)
    }
}
