use std::{env, fs::{self}};
use std::io::{self, Write};

mod utils;

use utils::{async_execute_file, not_found_err, remove_white_spaces};

const BUILT_IN_COMMANDS: [&str; 5] = ["echo", "exit", "type", "pwd", "cat"];

fn main() {
    // define vars
    let stdin = io::stdin();
    let mut input;
    let mut command;
    let mut args;
    loop {
        // initiate terminal
        print!("$ ");
        io::stdout().flush().unwrap();

        // take input
        input = String::new();
        stdin.read_line(&mut input).unwrap();

        let split = input.split_once(" ");

        match split {
            Some(str) => {
                command = str.0;
                args = str.1;
            },
            None => {
                command = input.trim();
                args = "";
            }
        }

        // // collect all args with command at args[0]
        let commands: Vec<&str> = input.split_ascii_whitespace().collect();

        // check for type of commands
        match command {
            "echo" => echo_command(args),
            "cat" => cat_command(args),
            "type" => type_command(commands),
            "pwd" => pwd_command(),
            "cd" => change_directory_command(commands),
            "exit" => {
                if exit_command(commands) {
                    break;
                }
            },
            _ => execute_files_command(commands),
        }
    }
}

fn type_command(commands: Vec<&str>) {
    let mut paths: Vec<&str> = [].to_vec();
    let p: String;
    // Get PATH from env vars
    match env::var("PATH") {
        Ok(path) => {
            p = path.clone();
            paths = p.split(":").collect();
        },
        Err(e) => println!("Couldn't read PATH: {}", e),
    }
    if BUILT_IN_COMMANDS.contains(&&commands[1])  {
        println!("{} is a shell builtin", commands[1]);
        return;
    }
    let mut is_found: bool = false;
    for path in paths {
        if is_found {
            break;
        }
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            if let Some(file_name) = entry.path().file_stem() {
                                if file_name == commands[1] {
                                    println!("{} is {}/{}", commands[1],path,file_name.to_string_lossy());
                                    is_found = true;
                                    break;
                                }
                            }
                        }
                        Err(e) => {},
                    }
                }
            },
            Err(e) => {},
        }
    }
    if !is_found {
        // not_found_err(commands, 1);
        println!("{}: not found ", commands[1]);
    }
}

fn execute_files_command(commands: Vec<&str>) {
    let mut paths: Vec<&str> = [].to_vec();
    let p: String;
    // Get PATH from env vars
    match env::var("PATH") {
        Ok(path) => {
            p = path.clone();
            paths = p.split(":").collect();
        },
        Err(e) => println!("Couldn't read PATH: {}", e),
    }
    let mut is_found: bool = false;
    for path in paths {
        if is_found {
            break;
        }
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            if let Some(file_name) = entry.path().file_stem() {
                                if file_name == commands[0] {
                                    async_execute_file( &commands,"");
                                    is_found = true;
                                    break;
                                }
                            }
                        }
                        Err(e) => {},
                    }
                }
            },
            Err(e) => {},
        }
    }
    if !is_found {
        not_found_err(commands, 0);
        // println!("{}: not found ", commands[1]);
    }
}

fn exit_command(commands: Vec<&str>) -> bool {
    if commands.len() >=2 && commands[1] == "0" {
        return true
    }
    not_found_err(commands, 1);
    false
}

fn pwd_command() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => println!("{}", e),
    }
}

fn change_directory_command(commands: Vec<&str>) {
    if commands[1] == "~" {
        match env::var("HOME") {
            Ok(path) => {
                match env::set_current_dir(path) {
                    Ok(_) => {},
                    Err(_) => {},
                }
            },
            Err(e) => println!("Couldn't read PATH: {}", e),
        }
        return;
    }
    match env::set_current_dir(commands[1]) {
        Ok(_) => {},
        Err(_) => println!("cd: {}: No such file or directory", commands[1]),
    }
}

fn echo_command(mut input: &str) {
    input = input.trim();
    let mut ans: Vec<&str>;
    if input[0..1].to_string() == "'" {
        ans = input.split("'").collect();
    } else if input[0..1].to_string() == "\"" { 
        ans = input.split("\"").collect();
    } else {
        ans = input.split_ascii_whitespace().collect();
        let m = input.to_string();
        for item in ans {
            if item.starts_with("\\") == true {
                continue;
            }
        }
        // println!("{}", ans.join(" "));
        return
    }
    ans = remove_white_spaces(&ans);
    println!("{}", ans.join(""));
}

fn cat_command(input: &str) {
    let file_names: Vec<&str>;
    if input[0..1].to_string() == "'" {
        file_names = input.split("'").collect();
    } else if input[0..1].to_string() == "\"" { 
        file_names = input.split("\"").collect();
    } else {
        file_names = input.split_whitespace().collect();
    }
    async_execute_file(&file_names, "cat");
}