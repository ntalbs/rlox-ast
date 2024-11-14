use std::fs::read_to_string;
use std::io::{stdin, stdout, Write};
use std::process::exit;
use std::{env, io};

mod token;

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

        match line.chars().next() {
            None => {
                println!();
                return Ok(());
            }
            Some('\n') => eprintln!("Ctrl-C or Ctrl-D to quit.\n"),
            Some(_) => println!("{line}"),
        }
    }
}

fn run(source: String) {
    println!("Not yet, Just print: {source}");
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, at: &str, message: &str) {
    eprintln!("[line {line} ] Error {at}: {message}");
}