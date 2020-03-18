use std::env;

mod lox;
use lox::driver;

fn main() -> () {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() > 2 {
        println!("Usage: {} [script]", arguments[0]);
        std::process::exit(64)
    } else if arguments.len() == 2 {
        driver::run_file(&arguments[1]);
    } else {
        driver::run_prompt();
    }
}
