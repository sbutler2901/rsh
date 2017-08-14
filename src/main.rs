use std::io;
use std::io::Write;
use std::env;
use std::process::Command;
use std::process::ExitStatus;
use std::path::{ /*Path,*/ PathBuf };
use std::str::SplitWhitespace;
use std::os::unix::process::ExitStatusExt;
use std::fs;


// Built in commands to be implemented:
// 1. fg
// 2. bg
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

struct ShellDirectories {
    user_home: PathBuf,
    current: PathBuf,
    previous: PathBuf,
}

impl ShellDirectories {
    fn new() -> ShellDirectories {
        let new_shell_dir = ShellDirectories {
            user_home: PathBuf::new(),
            current: PathBuf::new(),
            previous: PathBuf::new(),
        };
        new_shell_dir
    }

    fn update_dirs(&mut self) {
        if let Ok(current_dir) = env::current_dir() {
            self.previous = PathBuf::from(self.current.as_path());
            self.current = current_dir;
        }
    }
}

/*
fn print_paths() {
    println!("Paths:");
    let key = "PATH";
    match env::var_os(key) {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                println!("'{}'", path.display());
            }
        }
        None => println!("{} is not defined in the enviornment.", key)
    }
}*/

fn which(cmd: Option<&str>) {
    if let Some(cmd_unwrapped) = cmd {
        match cmd_unwrapped {
            "fg" | "bg" | "which" | "pushd" | "popd" | "dirs"
                => { println!("{} : shell builtin command", cmd_unwrapped); },
            _ => {
                if let Ok(mut child) = Command::new("/usr/bin/which")
                                                .arg(cmd_unwrapped)
                                                .spawn() 
                {
                    if let Err(e) = child.wait() {
                        println!("Error waiting for child: {}", e);
                    }
                } else {
                    println!("Error executing /usr/bin/which");
                }
            }
        };
    }
}

fn print_working_directory(shell_dirs: &ShellDirectories) {
    println!("{}", shell_dirs.current.display());
}

// TODO - Create error to be handled in case it is not a dir path
fn is_dir_path(dir_path: &str) -> bool {
    if let Ok(metadata) = fs::metadata(dir_path) {
        metadata.is_dir()
    } else {
        let path_buf = PathBuf::from(dir_path);
        if path_buf.is_relative() {
            true
        } else {
            false
        }
    }
}

fn cd(shell_dirs: &mut ShellDirectories, path_wrapped: Option<&str>) /*-> ExitStatus*/ {
    if let Some(path_unwrapped) = path_wrapped {
        if is_dir_path(path_unwrapped) {
            let temp_path_buf: PathBuf;
            let path = match path_unwrapped {
                "." => { shell_dirs.current.as_path() },
                ".." => { 
                    // FIXME - panics when this is provided and in the root directory
                    shell_dirs.current.parent().unwrap() 
                },
                "~" => { shell_dirs.user_home.as_path() },
                "-" => { shell_dirs.previous.as_path() },
                _ => { 
                    temp_path_buf = PathBuf::from(path_unwrapped);
                    temp_path_buf.as_path()
                },
            };
            if let Err(e) = env::set_current_dir(path) {
                println!("Error changing directory: {}", e);
            }
        }
    } else {
        println!("Please enter an appropriate path");
    }
    shell_dirs.update_dirs();

    //let exitStatus = ExitStatusExt::from_raw(0);
    //exitStatus
}

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

fn setup_shell_dirs(shell_dirs: &mut ShellDirectories) {
    if let Ok(current_dir) = env::current_dir() {
        shell_dirs.current = current_dir;
    }
    if let Some(user_home) = env::home_dir() {
        shell_dirs.user_home = user_home;
        shell_dirs.previous = PathBuf::from(shell_dirs.user_home.as_path());
    }
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

fn pushd(pushed_dirs: &mut Vec<PathBuf>, shell_dirs: &mut ShellDirectories, path: Option<&str>) {
    if let Some(dir_path) = path {
        if is_dir_path(dir_path) {
            pushed_dirs.push(PathBuf::from(shell_dirs.current.as_path()));
            cd(shell_dirs, path);
            dirs(&pushed_dirs);
        } else {
            println!("pushd: {} is not a directory", dir_path);
        }
    }
}

fn popd(pushed_dirs: &mut Vec<PathBuf>, shell_dirs: &mut ShellDirectories) {
    if let Some(popped_dir) = pushed_dirs.pop() {
        cd(shell_dirs, popped_dir.to_str());
        dirs(&pushed_dirs);
    } else {
        println!("popd: directory stack is empty");
    }
}

fn print_left_prompt(shell_dirs: &ShellDirectories) {
    print!("{}> ", shell_dirs.current.display());
    io::stdout().flush().unwrap();
}


fn main() {
    println!("Welcome to R(ust)Shell");
//    print_paths();

    let mut shell_dirs = ShellDirectories::new();
    let mut pushed_dirs: Vec<PathBuf> = Vec::new();     //TODO - Add limit to stack
    setup_shell_dirs(&mut shell_dirs); 

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
                    cd(&mut shell_dirs, dir_path);
                 },
                 "pwd" => {
                     print_working_directory(&shell_dirs);
                },
                "which" => {
                    let second_cmd = cmd_str_iter.next();
                    which(second_cmd);
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
                "exit" | "quit" => { break; },
                _ => {
                    let exit_status = exec_cmd(&cmd_unwrapped, &mut cmd_str_iter);
                    println!("{} - {}", cmd_unwrapped, exit_status);
                },
            };
        }
    }
}
