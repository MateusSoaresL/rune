use crate::frontend::ast::ast::{Expr, Program, Statement};
use crate::frontend::lexer::token::{Token, TokenKind};
// Is the principal parser structure, and save some information's parser.
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    // Create instance.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,     // The tokens in a vector.
            current: 0, // The current index.
        }
    }

    // Just peek the code.
    fn peek(&self) -> &Token {
        // Peek and return the current token.
        &self.tokens[self.current]
    }

    // Take the previous token.
    fn previous(&self) -> &Token {
        // Take the current token - 1, to return the previous token.
        &self.tokens[self.current - 1]
    }

    // Check if is at end.
    fn is_at_end(&self) -> bool {
        // Just peek and if is EOF, return EOF.
        self.peek().kind == TokenKind::EOF
    }

    // Just check the current token.
    fn check(&self, kind: TokenKind) -> bool {
        // Just check if the current token is the expected token.
        self.peek().kind == kind
    }

    // Advance the current index.
    fn advance(&mut self) -> &Token {
        // Check if is at end.
        if !self.is_at_end() {
            self.current += 1; // If is not, advance the current index.
        }

        // Return the previous token.
        self.previous()
    }

    // Consume the token.
    fn consume(&mut self, kind: TokenKind) -> Result<(), ()> {
        let token = self.peek(); // The token representation.

        // Will check the token.
        if self.check(kind) {
            self.advance(); // Advance.
            return Ok(()); // Return.
        }

        // Return token error and show the line and column.
        eprintln!("Unexpected token! At {}:{}", token.line, token.column);
        Err(())
    }

    // Parser read the expression and transforms her in AST's Expr.
    fn expression(&mut self) -> Result<Expr, ()> {
        // Take the expression and advance.
        let token = self.advance();

        // Will check the token kind.
        match &token.kind {
            // Will check the string literal.
            TokenKind::StringLiteral(value) => Ok(Expr::String(value.clone())),

            // Identifier.
            TokenKind::Identifier(name) => Ok(Expr::Variable(name.clone())),

            // Return token error and show the line in Parser.
            _ => {
                eprintln!("Invalid expression! At {}:{}", token.line, token.column);
                Err(())
            }
        }
    }

    // This function will be responsible about the '__print' (NativePrint).
    fn __print_statement(&mut self) -> Result<Statement, ()> {
        self.advance(); // Advance the '__print'.
        self.consume(TokenKind::LeftParen)?; // Consume the '('.

        // Will check the string literal.
        let value = self.expression()?;

        self.consume(TokenKind::RightParen)?; // Consume the ')'.
        self.consume(TokenKind::Semicolon)?; // Consume the ';'.

        Ok(Statement::NativePrint(value)) // Return the Native Print.
    }

    // This function will be responsible about the '__println' (NativePrintln).
    fn __println_statement(&mut self) -> Result<Statement, ()> {
        self.advance(); // Advance the '__println'.
        self.consume(TokenKind::LeftParen)?; // Consume the '('.

        // Will check the string literal.
        let value = self.expression()?;

        self.consume(TokenKind::RightParen)?; // Consume the ')'.
        self.consume(TokenKind::Semicolon)?; // Consume the ';'.

        Ok(Statement::NativePrintln(value)) // Return the Native Println.
    }

    // The variable declaration.
    fn variable_declaration(&mut self, mutable: bool) -> Result<Statement, ()> {
        // Consume 'let' or 'var'.
        self.advance();

        // Next token need be a 'identifier'.
        let name = {
            // For token's informations.
            let token = self.peek();

            match &token.kind {
                TokenKind::Identifier(name) => name.clone(),

                _ => {
                    eprintln!("Expected variable name at {}:{}", token.line, token.column);
                    return Err(());
                }
            }
        };

        // The 'peek()' borrow is finished, so we can use again.
        self.advance();

        // '='.
        self.consume(TokenKind::Equal)?;

        // Variable value.
        let value = self.expression()?;

        // ';'.
        self.consume(TokenKind::Semicolon)?;

        // Returns.
        Ok(Statement::VariableDeclaration {
            name,
            value,
            mutable,
        })
    }

    fn assignment_statement(&mut self) -> Result<Statement, ()> {
        // Take the variable name.
        let name = {
            let token = self.peek();

            match &token.kind {
                TokenKind::Identifier(name) => name.clone(),

                _ => {
                    eprintln!("Expected variable name at {}:{}", token.line, token.column);

                    return Err(());
                }
            }
        };

        // Consume the Identifier.
        self.advance();

        // Consume '='.
        self.consume(TokenKind::Equal)?;

        // Read the new value.
        let value = self.expression()?;

        // Consume ';'.
        self.consume(TokenKind::Semicolon)?;

        // Returns.
        Ok(Statement::Assignment { name, value })
    }

    // This function will be responsible about to valid the statements.
    fn statement(&mut self) -> Result<Statement, ()> {
        // Will check the statements.
        if self.check(TokenKind::NativePrint) {
            return self.__print_statement(); // return the '__print_statement();'.
        }

        if self.check(TokenKind::NativePrintln) {
            return self.__println_statement(); // return the '__println_statement();'.
        }

        if self.check(TokenKind::Let) {
            return self.variable_declaration(false); // let = variable imutable.
        }

        if self.check(TokenKind::Var) {
            return self.variable_declaration(true); // var = variable mutable.
        }

        if matches!(&self.peek().kind, TokenKind::Identifier(_)) {
            return self.assignment_statement();
        }

        let token = self.peek();

        // Return token error and show the line in Parser.
        eprintln!("Unknwon statement at {}:{}!", token.line, token.column);

        // Returns the error, if have error.
        Err(())
    }

    // The principal parser function.
    pub fn parse(&mut self) -> Result<Program, ()> {
        // This variable represents the all statements.
        let mut statements = Vec::new();

        // Will check if is at end.
        while !self.is_at_end() {
            // If is not, push the statements.
            statements.push(self.statement()?);
        }

        Ok(Program { statements }) // Return the statements in program.
    }
}
