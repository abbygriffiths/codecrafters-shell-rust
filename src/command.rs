use std::io;

pub enum Command {
    Exit,
    Unknown,
    Echo(String)
}

pub fn parse_command(line: &str) -> Result<Command, io::Error> {
    let tokens: Vec<_> = line.split(|ch: char| ch.is_whitespace()).collect();

    let command = *tokens.first().expect(&format!(
        "Could not get the first token from command: {}",
        line
    ));

    match command {
        "exit" => Ok(Command::Exit),
        "echo" => Ok(Command::Echo(tokens[1..].join(" "))),
        _ => Ok(Command::Unknown),
    }
}
