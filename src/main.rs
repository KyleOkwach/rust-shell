#[allow(unused_imports)]
use std::io::{self, Write};

enum Command {
    CmdEcho(String),
    CmdInvalid(String),
    CmdExit,
}

fn get_command(s: &str) -> Command {
    let line_collection: Vec<&str> = s.split_whitespace().collect();
    let cmd = line_collection[0];
    match cmd {
        "echo"  => Command::CmdEcho(line_collection[1..].join(" ")),
        "exit"  => Command::CmdExit,
        _       => Command::CmdInvalid(String::from(cmd)),
    }
}


fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .unwrap();
        
        let command = get_command(&input.as_str());

        match command {
            Command::CmdEcho(text) => {
                println!("{text}");
            },
            Command::CmdInvalid(cmd) => {
                println!("{}: command not found", cmd)
            }
            Command::CmdExit => {
                break
            }
        }
        
    }
}
