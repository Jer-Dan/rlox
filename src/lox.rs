use crate::{
    parser::Parser,
    scanner::Scanner,
    token::{Token, TokenType},
};
use std::{env, fs, io};

pub(crate) fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut lox = Lox::new();

    match args.len() {
        3.. => panic!("Usage: jlox [script]"),
        2 => lox.run_file(&args[1]),
        _ => lox.run_prompt(),
    }
}

pub struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Self {
        Lox { had_error: false }
    }

    fn run_file(&mut self, path: &str) {
        let file_contents = fs::read_to_string(path).unwrap();
        self.run(&file_contents);
    }

    fn run_prompt(&mut self) {
        loop {
            print!("> ");

            let line = {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to receive input");
                input
            };

            if line.is_empty() {
                break;
            }

            self.run(&line);
        }
    }

    fn run(&mut self, source: &str) {
        println!("{}", source);

        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();

        let mut parser = Parser::new(self, scanner.tokens());
        let expression = parser.parse().unwrap();
        expression.print(0);
    }

    pub fn error(&mut self, token: Token, message: &str) {
        match token.token_type {
            TokenType::Eof => self.report(token.line, " at end", message),
            _ => self.report(token.line, &format!(" at '{}'", token.lexeme), message),
        }
    }

    fn report(&mut self, line: usize, where_err: &str, message: &str) {
        self.had_error = true;
        eprintln!("[line {}] Error{}: {}", line, where_err, message);
    }
}
