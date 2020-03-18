use crate::lox::scanner;
use crate::lox::tokens;

use std::io::Write;

pub fn run(source: &String) -> () {
    let scanner: scanner::Scanner = scanner::Scanner::new(source.chars().peekable());
    let _: Vec<tokens::Token> = scanner
        .filter_map(|t| match t {
            Ok(token) => {
                if matches!(token.token_type, tokens::TokenType::Comment(_)) {
                    None
                } else {
                    println!("{}", token);
                    Some(token)
                }
            }
            Err(why) => {
                error(why);
                None
            }
        })
        .collect();
}

pub fn run_read<R: std::io::prelude::Read>(source: &mut R) -> () {
    // This is probably a bad idea. For now, assume file sizes are small enough
    let mut contents: String = String::new();
    match source.read_to_string(&mut contents) {
        Err(why) => error(why),
        Ok(_) => {
            run(&contents);
        }
    }
}

pub fn run_prompt() -> () {
    print!("> ");
    let _ = std::io::stdout().flush();
    let stdin: std::io::Stdin = std::io::stdin();
    let mut line: String = String::new();
    loop {
        // No multiline strings in repl mode for now
        match stdin.read_line(&mut line) {
            Ok(_) => {
                println!("Got token: {}", line);
                run(&line);
                line.clear();
                print!("> ");
                let _ = std::io::stdout().flush();
            }
            Err(why) => {
                error(why);
            }
        }
    }
}

pub fn run_file(filename: &str) -> () {
    let file: std::fs::File = match std::fs::File::open(filename) {
        Err(why) => {
            eprintln!("IO Error: {}", why);
            std::process::exit(65)
        }
        Ok(file) => file,
    };
    let mut reader = std::io::BufReader::new(file);
    run_read(&mut reader);
}

pub fn error<E: std::fmt::Display>(why: E) -> () {
    eprintln!("{}", why);
}
