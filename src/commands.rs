use std::env;
use std::path::Path;
use std::io::Error;
use crate::executable;

pub enum Command<'a> {
    CmdEcho(String),
    CmdInvalid(String),
    CmdExecutable(String, Vec<&'a str>),
    CmdType(String),
    CmdPwd,
    CmdCd(String),
    CmdExit,
}

pub const BUILTIN: &[&str] = &["echo", "exit", "type", "pwd", "cd"];

pub fn parse(s: &str) -> Command<'_> {
    let line_collection: Vec<&str> = s.split_whitespace().collect();

    let cmd = line_collection[0];
    let args: Vec<&str> = line_collection[1..].to_vec();
    
    match cmd {
        "echo"  => Command::CmdEcho(line_collection[1..].join(" ")),
        "exit"  => Command::CmdExit,
        "type"  => Command::CmdType(String::from(line_collection[1])),
        "pwd"   => Command::CmdPwd,
        "cd"    => Command::CmdCd(line_collection[1..].join(" ")),
        _       => {
            if let Some(path) = executable::find(cmd) {
                let path_string = path.to_string_lossy().into_owned();
                Command::CmdExecutable(path_string, args)
            } else {
                Command::CmdInvalid(String::from(cmd))
            }
        },
    }
}

pub fn print_working_directory() {
    let curr_dir = env::current_dir().unwrap();

    if let Some(path_str) = curr_dir.to_str() {
        println!("{}", path_str);
    }
}

pub fn change_directory(target_path: &str) {
    let root = Path::new(target_path);

    match env::set_current_dir(&root) {
        Ok(_) => {
            let Ok(current) = env::current_dir() else { todo!() };
        }
        Err(e) => {
            eprintln!("cd: '{}': No such file or directory", target_path);
        }
    }}
