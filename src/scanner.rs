use std::collections::HashMap;

use crate::token::{Token, TokenType};

pub(crate) struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: usize,
    keywords_map: HashMap<&'a str, TokenType<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,

            keywords_map: HashMap::from([
                ("and", TokenType::And),
                ("class", TokenType::Class),
                ("else", TokenType::Else),
                ("false", TokenType::False),
                ("for", TokenType::For),
                ("fun", TokenType::Fun),
                ("if", TokenType::If),
                ("nil", TokenType::Nil),
                ("or", TokenType::Or),
                ("print", TokenType::Print),
                ("return", TokenType::Return),
                ("super", TokenType::Super),
                ("this", TokenType::This),
                ("true", TokenType::True),
                ("var", TokenType::Var),
                ("while", TokenType::While),
            ]),
        }
    }

    fn get_keyword_tokentype(&self, raw: &'a str) -> TokenType<'a> {
        self.keywords_map
            .get(raw)
            .unwrap_or(&TokenType::Identifier(raw))
            .clone()
    }

    pub fn tokens(&self) -> &Vec<Token<'a>> {
        &self.tokens
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, "", self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        if let Some(c) = self.advance() {
            let token_type = match c {
                '(' => TokenType::LeftParen,
                ')' => TokenType::RightParen,
                '{' => TokenType::LeftBrace,
                '}' => TokenType::RightBrace,
                ',' => TokenType::Comma,
                '.' => TokenType::Dot,
                '-' => TokenType::Minus,
                '+' => TokenType::Plus,
                ';' => TokenType::SemiColon,
                '*' => TokenType::Star,
                '!' => {
                    if self.match_char('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    }
                }
                '=' => {
                    if self.match_char('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    }
                }
                '<' => {
                    if self.match_char('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    }
                }
                '>' => {
                    if self.match_char('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    }
                }
                '/' => {
                    if self.match_char('/') {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                        TokenType::Comment
                    } else {
                        TokenType::Slash
                    }
                }
                ' ' | '\r' | '\t' => TokenType::Whitespace,
                '\n' => {
                    self.line += 1;
                    TokenType::Whitespace
                }
                '"' => TokenType::String(self.consume_str()),
                _ if c.is_digit(10) => TokenType::Number(self.consume_num()),
                _ if c.is_alphabetic() => self.consume_identifier(),
                _ => TokenType::Error,
            };

            if token_type == TokenType::Error {
                self.error("Unexpected character.");
            }

            if ![TokenType::Comment, TokenType::Whitespace].contains(&token_type) {
                self.add_token(token_type);
            }
        } else {
            self.error("Invalid token (this message shouldn't show up).");
        }
    }

    fn advance(&mut self) -> Option<char> {
        let next = self.source.chars().nth(self.current);
        if let Some(_) = next {
            self.current += 1
        };
        next
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap_or('\0')
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn add_token(&mut self, token_type: TokenType<'a>) {
        let lexeme = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, lexeme, self.line));
    }

    fn error(&self, message: &str) {
        panic!("Line {}: {}", self.line, message);
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn consume_str(&mut self) -> &'a str {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error("Unterminated string.");
        }

        self.advance();

        &self.source[self.start + 1..self.current - 1]
    }

    fn consume_num(&mut self) -> f32 {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        self.source[self.start..self.current]
            .parse::<f32>()
            .unwrap()
    }

    fn consume_identifier(&mut self) -> TokenType<'a> {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        self.get_keyword_tokentype(&self.source[self.start..self.current])
    }
}
