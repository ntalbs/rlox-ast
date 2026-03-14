use std::fs::read_to_string;
use std::io::{stdin, stdout, Write};
use std::process::exit;
use std::{env, io};

use crate::parser::Parser;
use crate::scanner::Scanner;

mod token;
mod scanner;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox-ast [script]");
        exit(64);
    } else if args.len() == 2 {
        let source_path = &args[1];
        run_file(source_path.as_str());
    } else {
        let _ = run_prompt();
    }
}

fn run_file(path: &str) {
    println!("run_file: {path}");
    run(read_to_string(path).unwrap());
}

fn run_prompt() -> io::Result<()> {
    loop {
        print!("> ");
        stdout().flush()?;

        let mut line = String::new();
        stdin().read_line(&mut line)?;
        run(line);
    }
}

fn run(source: String) {
    let mut scanner = Scanner::new(&source);
    let tokens = scanner.scan();
    for t in &tokens {
        println!("{t:?}");
    }

    let mut parser = Parser::new(tokens);
    let expr = parser.parse();
    println!("Expr: {expr:?}");
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, at: &str, message: &str) {
    eprintln!("[line {line} ] Error {at}: {message}");
}
