use std::io;
use std::io::Write;
use std::process::Command;
use std::process::ExitStatus;
use std::path::PathBuf;
use std::str::SplitWhitespace;
use std::os::unix::process::ExitStatusExt;
use std::error::Error;
use std::collections::HashMap;

mod builtins;
mod shelldirs;
mod utils;
mod dirpatherror;

use shelldirs::ShellDirs;
use utils::*;

// Built in commands to be implemented:
// 1. fg / bg / jobs
// 3. where
// 4. which*
// 5. echo
// 6. alias
// 7. pushd / popd / dirs
// 8. setopts
// 9. export
//
// Features to be implemented:
// 1. Start up files: rshrc, etc.
// 2. $PATH command options
//      a. * Refer to PathBuf::read_dir() as an implementation option
// 3. File / Dir globbering
//
// Helpful Notes:
// 1. Result: Ok(), etc
// 2. Option: Some(), None

fn exec_cmd(cmd: &str, cmd_str_iter: &mut SplitWhitespace) -> ExitStatus {
    let exit_status;
    if let Ok(mut child) = Command::new(cmd).args(cmd_str_iter).spawn() {
        if let Ok(status) = child.wait() {
            exit_status = status;
        } else {
            println!("Child was not running");
            exit_status = ExitStatusExt::from_raw(-1);
        }
    } else {
        println!("{} failed to spawn", cmd);
        exit_status = ExitStatusExt::from_raw(-1);
    }
    exit_status
}

fn get_input(input_str: &mut String) {
    io::stdin().read_line(input_str).expect("Failed to read line");
}

fn print_left_prompt(shell_dirs: &shelldirs::ShellDirs) {
    print!("{}> ", shell_dirs.current.display());
    io::stdout().flush().unwrap();
}

fn main() {
    println!("Welcome to R(ust)Shell");
//    print_paths();

    let mut shell_dirs = shelldirs::ShellDirs::new();
    let mut pushed_dirs: Vec<PathBuf> = Vec::new();     //TODO - Add limit to stack
    let mut aliases: HashMap<String, String> = HashMap::new();

    ShellDirs::setup(&mut shell_dirs); 

    loop {
        print_left_prompt(&shell_dirs);

        let mut input_str = String::new();
        get_input(&mut input_str);

        let mut cmd_str_iter = input_str.trim().split_whitespace();

        let cmd_wrapped = cmd_str_iter.next();
        if let Some(cmd_unwrapped) = cmd_wrapped {
            let exit_status;
            if let Some(alias_cmd) = builtins::alias::get_aliased(&aliases.clone(), &cmd_unwrapped) {
                exit_status = exec_cmd(&alias_cmd, &mut cmd_str_iter);
                println!("{} - {}", cmd_unwrapped, exit_status);
            } else {
                match cmd_unwrapped {
                    "alias" => {
                        if let Some(mapping) = cmd_str_iter.next() {
                            if let Some(equal_index) = mapping.find("='") {
                                // Only accepts well formed input "<alias>='<mapping>'"
                                let (key, tmp_value_0) = mapping.split_at(equal_index);
                                let tmp_value_1 = tmp_value_0.replacen("='", "", 1);
                                let value = tmp_value_1.replace("'", "");
                                if let Err(e) = builtins::alias::alias(&mut aliases, key, value) {
                                    println!("alias: {}", e);
                                }
                            } else {
                                println!("alias: bad mapping");
                            }
                        } else {
                            builtins::alias::show_aliases(&aliases);
                        }
                    },
                    "cd" => {
                        let dir_path;
                        if let Some(received_path) = cmd_str_iter.next() {
                            let orig_path = PathBuf::from(received_path);
                            dir_path = relative_to_absolute(&shell_dirs, &orig_path);
                            if let Err(e) = builtins::cd::cd(&mut shell_dirs, &dir_path) {
                                println!("cd: {}", e.description());
                            }
                        } else {
                            dir_path = shell_dirs.user_home.clone();
                            if let Err(e) = builtins::cd::cd(&mut shell_dirs, &dir_path) {
                                println!("cd: {}", e.description());
                            }
                        }
                    },
                    "dirs" => {
                        builtins::dirstack::dirs(&pushed_dirs);
                    },
                    "popd" => {
                        if let Err(e) = builtins::dirstack::popd(&mut pushed_dirs, &mut shell_dirs) {
                            println!("popd: {}", e.description());
                        }
                    },
                    "pushd" => {
                        if let Some(received_path) = cmd_str_iter.next() {
                            let orig_path = PathBuf::from(received_path);
                            let dir_path = relative_to_absolute(&shell_dirs, &orig_path);
                            if let Err(e) = builtins::dirstack::pushd(&mut pushed_dirs, &mut shell_dirs, &dir_path) {
                                println!("pushd: {}", e.description());
                            }
                        }
                    },
                    "pwd" => {
                         builtins::pwd::pwd(&shell_dirs);
                    },
                    "which" => {
                        let second_cmd = cmd_str_iter.next();
                        builtins::which::which(second_cmd);
                    },
                    "fg" | "bg" | "jobs" => {
                        println!("TODO");
                    },
                    "exit" => { break; },
                    _ => {
                        exit_status = exec_cmd(&cmd_unwrapped, &mut cmd_str_iter);
                        println!("{} - {}", cmd_unwrapped, exit_status);
                    },
                };
            }
        }
    }
}
