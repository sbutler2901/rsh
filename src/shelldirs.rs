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

    pub fn update_dirs(&mut self) {
        if let Ok(current_dir) = env::current_dir() {
            self.previous = PathBuf::from(self.current.as_path());
            self.current = current_dir;
        }
    }
}
