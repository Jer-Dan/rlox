use crate::scanner::Scanner;
use std::{env, fs, io};

pub(crate) fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let lox = Lox::new();

    match args.len() {
        3.. => panic!("Usage: jlox [script]"),
        2 => lox.run_file(&args[1]),
        _ => lox.run_prompt(),
    }
}

struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Self {
        Lox { had_error: false }
    }

    fn run_file(&self, path: &str) {
        let file_contents = fs::read_to_string(path).unwrap();
        self.run(&file_contents);
    }

    fn run_prompt(&self) {
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

    fn run(&self, source: &str) {
        println!("{}", source);

        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();

        for token in scanner.tokens() {
            println!("{}", token.to_string());
        }
    }

    fn error(&mut self, line: i32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: i32, where_err: &str, message: &str) {
        self.had_error = true;
        panic!("[line {}] Error{}: {}", line, where_err, message);
    }
}
