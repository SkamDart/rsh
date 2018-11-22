use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
struct Command {
    command: String,
    args: Vec<String>
}

pub fn execute(command: String) {
    println!("Executing {}", command);
}

pub fn print_prompt() {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write(b"> ").unwrap();
    handle.flush();
}

pub fn wait_for_command() -> String {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    return user_input;
}

// touch
pub fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

// cat
pub fn cat(path: &Path) -> io::Result<()> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
