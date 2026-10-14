#[allow(unused_imports)]
mod executable;
mod command;

use std::collections::HashSet;
use std::io::{self, Write};
use command::Command;

fn main() {
    let builtin_commands: HashSet<&str> = vec!["echo", "exit", "type", "pwd"].into_iter().collect();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .unwrap();
        
        let command = command::parse(&input.as_str());

        match command {
            Command::CmdEcho(text) => {
                println!("{text}");
            },
            Command::CmdPwd => {
                let curr_dir = std::env::current_dir().unwrap();
                println!("{:?}", curr_dir);
            }
            Command::CmdInvalid(cmd) => {
                println!("{}: command not found", cmd)
            },
            Command::CmdType(cmd) => {
                if builtin_commands.contains(cmd.as_str()) {
                    println!("{} is a shell builtin", cmd)
                } else if let Some(path) = executable::find(cmd.as_str()){
                    // Get path
                    println!("{} is {}", cmd, path.display());
                } else {
                    println!("{}: not found", cmd)
                }
            },
            Command::CmdExecutable(path, args) => {
                use std::os::unix::process::CommandExt;  // Override argv[0]

                // Extract base file from path such as "/usr/bin/cargo" -> "cargo"
                let base_name = path.split('/').last().unwrap_or(&path);

                let child = std::process::Command::new(&path)
                    .arg0(base_name)
                    .args(&args)
                    .spawn();

                match child {
                    Ok(mut process) => {
                        let _ = process.wait();
                    }
                    Err (e) => {
                        eprintln!("Error executing command: {}", e);
                    }
                }
            }
            Command::CmdExit => {
                break
            },
        }
        
    }
}
