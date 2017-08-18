use std::path::PathBuf;
use std::env;

use shelldirs::ShellDirs;

// Assumes dir_path is an absolute path
pub fn cd(shell_dirs: &mut ShellDirs, dir_path: &PathBuf) {
    if let Err(e) = env::set_current_dir(dir_path) {
        println!("Error changing directory: {}", e);
    } else {
        let new_current = dir_path.clone();
        shell_dirs.update_dirs(new_current);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
/*
    #[test]
    fn cd_handles_relative_current() {
        let mut shell_dirs = ShellDirs::new();
        ShellDirs::setup(&mut shell_dirs);

        // Verify current relative
        let current = shell_dirs.current.clone();
        cd(&mut shell_dirs, &PathBuf::from("."));
        assert_eq!(shell_dirs.current, current);
        assert_eq!(shell_dirs.previous, current);

        let current = shell_dirs.current.clone();
        cd(&mut shell_dirs, &PathBuf::from("./"));
        assert_eq!(shell_dirs.current, current);
        assert_eq!(shell_dirs.previous, current);
    }

    #[test]
    fn cd_handles_relative_parent() {
        let mut shell_dirs = ShellDirs::new();
        ShellDirs::setup(&mut shell_dirs);

        let previous = shell_dirs.current.clone();
        cd(&mut shell_dirs, &PathBuf::from(".."));
        assert_eq!(shell_dirs.previous, previous);
        assert!(shell_dirs.current.as_path().eq(shell_dirs.previous.parent().unwrap()));
    }

    #[test]
    fn cd_handles_relative_parent_root() {
        let mut shell_dirs = ShellDirs::new();
        ShellDirs::setup(&mut shell_dirs);

        let mut path = "/";
        let previous = shell_dirs.current.clone();
        cd(&mut shell_dirs, &PathBuf::from(path));
        assert_eq!(shell_dirs.current, PathBuf::from(path));
        assert_eq!(shell_dirs.previous, previous);

        path = "..";
        cd(&mut shell_dirs, &PathBuf::from(path));
        assert_eq!(shell_dirs.current, PathBuf::from("/"));
        assert_eq!(shell_dirs.previous, PathBuf::from("/"));
    }

    #[test]
    fn cd_handles_relative_previous() {
        let mut shell_dirs = ShellDirs::new();
        ShellDirs::setup(&mut shell_dirs);

        let mut path = "-";
        let mut previous = shell_dirs.previous.clone();
        cd(&mut shell_dirs, &PathBuf::from(path));
        assert_eq!(shell_dirs.current, previous);

        path = "/tmp";
        previous = shell_dirs.current.clone();
        cd(&mut shell_dirs, &PathBuf::from(path));
        assert_eq!(shell_dirs.current, PathBuf::from(path));
        assert_eq!(shell_dirs.previous, previous);

        path = "-";
        previous = shell_dirs.previous.clone();
        cd(&mut shell_dirs, &PathBuf::from(path));
        assert_eq!(shell_dirs.current, previous);
        assert_eq!(shell_dirs.previous, PathBuf::from("/tmp"));
    }
*/
    #[test]
    fn cd_handles_absolute() {
        let mut shell_dirs = ShellDirs::new();
        ShellDirs::setup(&mut shell_dirs);

        let path = "/etc";
        let previous = shell_dirs.current.clone();
        cd(&mut shell_dirs, &PathBuf::from(path));
        assert_eq!(shell_dirs.current, PathBuf::from(path));
        assert_eq!(shell_dirs.previous, previous);
    }
 
    #[test]
    fn cd_handles_non_dir() {
        let mut shell_dirs = ShellDirs::new();
        ShellDirs::setup(&mut shell_dirs);
        
        let mut path = "index.html";
        let mut current = shell_dirs.current.clone();
        let mut previous = shell_dirs.previous.clone();
        cd(&mut shell_dirs, &PathBuf::from(path));
        assert_eq!(shell_dirs.current, current);
        assert_eq!(shell_dirs.previous, previous);

        path = "./index.html";
        current = shell_dirs.current.clone();
        previous = shell_dirs.previous.clone();
        cd(&mut shell_dirs, &PathBuf::from(path));
        assert_eq!(shell_dirs.current, current);
        assert_eq!(shell_dirs.previous, previous);

        path = "/tmp/index.html";
        current = shell_dirs.current.clone();
        previous = shell_dirs.previous.clone();
        cd(&mut shell_dirs, &PathBuf::from(path));
        assert_eq!(shell_dirs.current, current);
        assert_eq!(shell_dirs.previous, previous);
    }
}
