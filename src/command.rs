use crate::executable;

pub enum Command<'a> {
    CmdEcho(String),
    CmdInvalid(String),
    CmdExecutable(String, Vec<&'a str>),
    CmdType(String),
    CmdPwd,
    CmdExit,
}

pub fn parse(s: &str) -> Command<'_> {
    let line_collection: Vec<&str> = s.split_whitespace().collect();

    let cmd = line_collection[0];
    let args: Vec<&str> = line_collection[1..].to_vec();
    
    match cmd {
        "echo"  => Command::CmdEcho(line_collection[1..].join(" ")),
        "exit"  => Command::CmdExit,
        "type"  => Command::CmdType(String::from(line_collection[1])),
        "pwd"   => Command::CmdPwd,
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
