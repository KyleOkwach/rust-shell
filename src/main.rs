#[allow(unused_imports)]
use std::os::unix::fs::PermissionsExt;
use std::collections::HashSet;
use std::io::{self, Write};
use std::env;
use std::path::{Path, PathBuf};

enum Command {
    CmdEcho(String),
    CmdInvalid(String),
    CmdType(String),
    CmdExit,
}


fn get_command(s: &str) -> Command {
    let line_collection: Vec<&str> = s.split_whitespace().collect();
    let cmd = line_collection[0];
    match cmd {
        "echo"  => Command::CmdEcho(line_collection[1..].join(" ")),
        "exit"  => Command::CmdExit,
        "type"  => Command::CmdType(String::from(line_collection[1])),
        _       => Command::CmdInvalid(String::from(cmd)),
    }
}

fn find_executable<P: AsRef<Path>>(exe_name: P) -> Option<PathBuf> {
    // Get system's PATH environment variable
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .filter_map(|dir| {
                let full_path = dir.join(&exe_name);
                
                if let Ok(metadata) = full_path.metadata() {
                    let is_file = metadata.is_file();

                    let is_executable = (metadata.permissions().mode() & 0o111) != 0;

                    if is_file && is_executable {
                        return Some(full_path);
                    }
                }
                None
            }).next()
    })
}

fn main() {
    let builtin_commands: HashSet<&str> = vec!["echo", "exit", "type"].into_iter().collect();

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
            },
            Command::CmdType(cmd) => {
                if builtin_commands.contains(cmd.as_str()) {
                    println!("{} is a shell builtin", cmd)
                } else if let Some(path) = find_executable(cmd.as_str()){
                    // Get path
                    println!("{} is {}", cmd, path.display());
                } else {
                    println!("{}: not found", cmd)
                }
            },
            Command::CmdExit => {
                break
            },
        }
        
    }
}
