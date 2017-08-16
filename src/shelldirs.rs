use std::path::PathBuf;
use std::env;

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
        if let Some(user_home) = env::home_dir() {
            shell_dirs.user_home = user_home;
            shell_dirs.previous = PathBuf::from(shell_dirs.user_home.as_path());
        }
    }

    pub fn update_dirs(&mut self, new_current: Option<PathBuf>) {
        //if let Ok(current_dir) = env::current_dir() {
        if let Some(new) = new_current {
            self.previous = PathBuf::from(self.current.as_path());
            self.current = new;
            //self.current = current_dir;
        }
    }
}
