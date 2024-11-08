#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TokenType<'a> {
    // Single char
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // 1-2 char
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier(&'a str),
    String(&'a str),
    Number(f32),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Ignored
    Comment,
    Whitespace,

    Eof,

    Error,
}

#[derive(Clone, Debug)]
pub(crate) struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub lexeme: &'a str,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType<'a>, lexeme: &'a str, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }

    pub fn to_string(&self, indent: usize) -> String {
        format!(
            "Token: {:?}\n{}Lexeme: {}\n",
            self.token_type,
            " ".repeat(indent),
            self.lexeme
        )
    }
}
