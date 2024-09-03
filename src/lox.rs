use std::{env, fs, io};

pub(crate) fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2.. => panic!("Usage: jlox [script]"),
        1 => Lox::run_file(&args[0]),
        _ => Lox::run_prompt(),
    }
}

struct Lox {
    had_error: bool,
}

impl Lox {
    fn run_file(path: &str) {
        let file_contents = fs::read_to_string(path).unwrap();
        Self::run(&file_contents);
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
        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens();

        for token in tokens {
            println!("{:?}", token);
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
