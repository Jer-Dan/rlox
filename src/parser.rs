use crate::{
    expr::{Binary, Expression, Grouping, Literal, Unary},
    lox::Lox,
    token::{Token, TokenType},
};

pub struct Parser<'a> {
    lox: &'a mut Lox,
    tokens: &'a Vec<Token<'a>>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(lox: &'a mut Lox, tokens: &'a Vec<Token<'a>>) -> Self {
        Parser {
            lox,
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expression<'a>, &'static str> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expression<'a>, &'static str> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression<'a>, &'static str> {
        let mut expr = self.comparison()?;

        while self.match_types(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token<'a> = self.previous().clone();
            let right = self.comparison()?;
            expr = Expression::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }

        Ok(expr)
    }

    fn match_types(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type.clone()) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token<'a> {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token<'a> {
        self.tokens.get(self.current).unwrap()
    }

    fn consume(
        &mut self,
        token_type: TokenType,
        message: &'static str,
    ) -> Result<&Token<'a>, &'static str> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            let token = self.tokens.get(self.current).unwrap();

            self.error(token.clone(), message)
        }
    }

    fn previous(&self) -> &Token<'a> {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn comparison(&mut self) -> Result<Expression<'a>, &'static str> {
        let mut expr = self.term()?;

        while self.match_types(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expression::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression<'a>, &'static str> {
        let mut expr = self.factor()?;

        while self.match_types(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expression::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression<'a>, &'static str> {
        let mut expr = self.unary()?;

        while self.match_types(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expression::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression<'a>, &'static str> {
        if self.match_types(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            Ok(Expression::Unary(Unary::new(operator, Box::new(right))))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expression<'a>, &'static str> {
        match self.peek().token_type {
            TokenType::False
            | TokenType::True
            | TokenType::Nil
            | TokenType::Number(_)
            | TokenType::String(_) => {
                self.advance();
                Ok(Expression::Literal(Literal::new(self.previous().clone())))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
                Ok(Expression::Grouping(Grouping::new(Box::new(expr))))
            }
            _ => self.error(self.peek().clone(), "Expect expression"),
        }
    }

    fn error<T>(&mut self, token: Token, message: &'static str) -> Result<T, &'static str> {
        self.lox.error(token, message);
        Err(message)
    }

    fn synchronise(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SemiColon {
                return;
            }

            match self.peek().token_type {
                TokenType::Return => return,
                _ => {}
            };

            self.advance();
        }
    }
}
