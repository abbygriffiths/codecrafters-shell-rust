#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // REPL
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        input = input.trim().to_string();
        match input {
            _ => eprintln!("{}: command not found", input)
        }
    }
}
