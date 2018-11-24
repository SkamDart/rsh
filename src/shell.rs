
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::Path;

struct Command {
    name: String,
    args: Vec<String>,
}

// touch
pub fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

// cat
fn cat(path: &Path) -> io::Result<String> {
    // TODO - Don't panic. Handle gracefully.
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    println!("{:?}", s);
    Ok(s)
}

fn input_as_command(user_input: String) -> Command {
    let mut tokens = user_input.trim().split_whitespace();
    let name = tokens.next().unwrap().to_string();
    let args = tokens;
    let mut v = Vec::new();

    for arg in args {
        v.push(arg.clone().to_string());
    }

    Command {
        name: name,
        args: v,
    }
}

fn vec_as_str(vec: Vec<String>) -> String {
    let mut ret = String::new();
    for it in vec {
        ret.push_str(&it);
    }
    ret
}

pub fn handle(user_input: String) {
    let cmd: Command = input_as_command(user_input);
    // hax
    let path_str = vec_as_str(cmd.args);
    let path = Path::new(&path_str);
    // hax
    if cmd.name == "cat" {
        cat(path).unwrap();
    } else if cmd.name == "touch" {
        touch(path).unwrap();
    }
}

pub fn print_prompt() {
    // move to Session struct and start using implicit mutex lock on stdout fd.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write(b"> ").unwrap();
    handle.flush().unwrap();
}

pub fn wait_for_command() -> String {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    user_input.trim();
    return user_input;
}
