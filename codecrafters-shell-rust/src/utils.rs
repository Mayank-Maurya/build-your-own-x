use std::io::{self, Write};
use std::process::Command;

pub fn remove_white_spaces<'a>(ans: &[&'a str]) -> Vec<&'a str> {
    let mut result: Vec<&str> = [].to_vec();
    for item in ans {
        if item.len() == 0 {
            continue;
        }
        let mut trim = item.trim();
        if trim.len() == 0 {
            trim = " ";
        }
        result.push(trim);
    }
    result
}


pub fn async_execute_file(commands: &[&str], explicit_command: &str) {
    let cmd;
    if !explicit_command.is_empty() {
        cmd = explicit_command;
    } else {
        cmd = commands[0];
    }
    let output = Command::new(cmd)
        .args(commands[1..].iter())
        .output();
    io::stdout().write_all(&output.unwrap().stdout).unwrap();
}


pub fn not_found_err(commands: Vec<&str>, start_index: usize) {
    println!("{}: command not found ",commands[start_index..].join(" "));
}

