#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

pub mod command;
use command::{parse_command, Command};

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
        match parse_command(&input) {
            Ok(command) => match command {
                Command::Unknown => eprintln!("{}: command not found", input),
                Command::Exit => exit(0),
            },
            Err(e) => eprintln!("something went wrong: {}", e),
        }
    }
}
