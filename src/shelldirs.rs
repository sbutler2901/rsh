use std::path::PathBuf;
use std::env;

extern crate dirs;

pub struct ShellDirs {
    pub user_home: PathBuf,
    pub current: PathBuf,
    pub previous: PathBuf,
}

impl ShellDirs {
    pub fn new() -> ShellDirs {
        let new_shell_dir = ShellDirs {
            user_home: PathBuf::new(),
            current: PathBuf::new(),
            previous: PathBuf::new(),
        };
        new_shell_dir
    }

    pub fn setup(shell_dirs: &mut ShellDirs) {
        if let Ok(current_dir) = env::current_dir() {
            shell_dirs.current = current_dir;
        }
        if let Some(user_home) = dirs::home_dir() {
            shell_dirs.user_home = user_home;
            shell_dirs.previous = shell_dirs.user_home.clone();
        }
    }
}
