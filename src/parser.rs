use crate::{
    ast::{BinaryOperator, Expression, Statement},
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

    // The parser position.
    fn position(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    // Advance to the next token.
    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    // Parse an expression.
    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_additive()
    }

    // Parse addition and subtraction.
    fn parse_additive(&mut self) -> Result<Expression, String> {
        let mut expression = self.parse_multiplicative()?;

        loop {
            let operator = match self.position() {
                Some(Token {
                    kind: TokenKind::Plus,
                    ..
                }) => BinaryOperator::Add,

                Some(Token {
                    kind: TokenKind::Minus,
                    ..
                }) => BinaryOperator::Subtract,

                _ => break,
            };

            self.advance();

            let right = self.parse_multiplicative()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    // Parse multiplication and division.
    fn parse_multiplicative(&mut self) -> Result<Expression, String> {
        let mut expression = self.parse_primary()?;

        loop {
            let operator = match self.position() {
                Some(Token {
                    kind: TokenKind::Star,
                    ..
                }) => BinaryOperator::Multiply,

                Some(Token {
                    kind: TokenKind::Slash,
                    ..
                }) => BinaryOperator::Divide,

                _ => break,
            };

            self.advance();

            let right = self.parse_primary()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    // Parse primary expressions.
    fn parse_primary(&mut self) -> Result<Expression, String> {
        match self.position() {
            Some(Token {
                kind: TokenKind::StringLiteral(value),
                ..
            }) => {
                let value = value.clone();

                self.advance();

                Ok(Expression::StringLiteral(value))
            }

            Some(Token {
                kind: TokenKind::NumberLiteral(value),
                ..
            }) => {
                let value = *value;

                self.advance();

                Ok(Expression::NumberLiteral(value))
            }

            Some(Token {
                kind: TokenKind::Identifier(name),
                ..
            }) => {
                let name = name.clone();

                self.advance();

                Ok(Expression::Identifier(name))
            }

            Some(Token {
                kind: TokenKind::LeftParen,
                ..
            }) => {
                self.advance();

                let expression = self.parse_expression()?;

                self.expect(TokenKind::RightParen)?;

                Ok(expression)
            }

            Some(Token {
                kind: TokenKind::EOF,
                line,
                column,
            }) => Err(format!(
                "Expected expression, found end of file at {}:{}",
                line, column
            )),

            Some(token) => Err(format!(
                "Expected expression, found {:?} at {}:{}",
                token.kind, token.line, token.column
            )),

            None => Err("Unexpected parser state: missing EOF token".to_string()),
        }
    }

    // Expect a specific token.
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

    // Parse print and println.
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

    // Parse a statement.
    fn parse_statement(&mut self) -> Result<Statement, String> {
        let name = match self.position() {
            Some(Token {
                kind: TokenKind::Identifier(name),
                ..
            }) => name.clone(),

            Some(token) => {
                return Err(format!(
                    "Expected statement, found {:?} at {}:{}",
                    token.kind, token.line, token.column
                ));
            }

            None => {
                return Err("Unexpected parser state: missing EOF token".to_string());
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

    // Check if parser reached EOF.
    fn is_at_end(&self) -> bool {
        matches!(
            self.position(),
            Some(Token {
                kind: TokenKind::EOF,
                ..
            })
        )
    }

    // Main parser function.
    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            let statement = self.parse_statement()?;

            statements.push(statement);
        }

        Ok(statements)
    }
}
