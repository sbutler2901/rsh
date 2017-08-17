use std::io;
use std::io::Write;
//use std::env;
use std::process::Command;
use std::process::ExitStatus;
use std::path::{ /*Path,*/ PathBuf };
use std::str::SplitWhitespace;
use std::os::unix::process::ExitStatusExt;

mod builtins;
mod shelldirs;
mod utils;

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
//
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

fn dirs(pushed_dirs: &Vec<PathBuf>) {
    if !pushed_dirs.is_empty() {
        let pushed_reversed_iter = pushed_dirs.iter().rev();
        for dir in pushed_reversed_iter {
            print!("{} ", dir.display());
        }
        println!("");
    } else {
        println!("Directory stack is empty");
    }
}

fn pushd(pushed_dirs: &mut Vec<PathBuf>, shell_dirs: &mut ShellDirs, path: Option<&str>) {
    if let Some(dir_path) = path {
        if is_dir_path(dir_path) {
            pushed_dirs.push(PathBuf::from(shell_dirs.current.as_path()));
            builtins::cd::cd(shell_dirs, path);
            dirs(&pushed_dirs);
        } else {
            println!("pushd: {} is not a directory", dir_path);
        }
    }
}

fn popd(pushed_dirs: &mut Vec<PathBuf>, shell_dirs: &mut ShellDirs) {
    if let Some(popped_dir) = pushed_dirs.pop() {
        builtins::cd::cd(shell_dirs, popped_dir.to_str());
        dirs(&pushed_dirs);
    } else {
        println!("popd: directory stack is empty");
    }
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
    ShellDirs::setup(&mut shell_dirs); 

    loop {
        print_left_prompt(&shell_dirs);

        let mut input_str = String::new();
        get_input(&mut input_str);

        let mut cmd_str_iter = input_str.trim().split_whitespace();

        let cmd_wrapped = cmd_str_iter.next();
        if let Some(cmd_unwrapped) = cmd_wrapped {
            match cmd_unwrapped {
                "cd" => {
                    let dir_path = cmd_str_iter.next();
                    builtins::cd::cd(&mut shell_dirs, dir_path);
                 },
                 "pwd" => {
                     builtins::pwd::pwd(&shell_dirs);
                },
                "which" => {
                    let second_cmd = cmd_str_iter.next();
                    builtins::which::which(second_cmd);
                },
                "pushd" => {
                    let dir_path = cmd_str_iter.next();
                    pushd(&mut pushed_dirs, &mut shell_dirs, dir_path);
                },
                "popd" => {
                    popd(&mut pushed_dirs, &mut shell_dirs);
                },
                "dirs" => {
                    dirs(&pushed_dirs);
                },
                "exit" => { break; },
                _ => {
                    let exit_status = exec_cmd(&cmd_unwrapped, &mut cmd_str_iter);
                    println!("{} - {}", cmd_unwrapped, exit_status);
                },
            };
        }
    }
}
