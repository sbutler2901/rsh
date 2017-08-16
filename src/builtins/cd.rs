use std::path::PathBuf;
use std::env;

use shelldirs::ShellDirs;
use utils::is_dir_path;

pub fn cd(shell_dirs: &mut ShellDirs, path_wrapped: Option<&str>) /*-> ExitStatus*/ {
    let mut new_current: Option<PathBuf> = None;
    if let Some(path_unwrapped) = path_wrapped {
        if is_dir_path(path_unwrapped) {
            let temp_path_buf: PathBuf;
            let path = match path_unwrapped {
                "." | "./" => { shell_dirs.current.as_path() },
                ".." => { 
                    if let Some(parent) = shell_dirs.current.parent() {
                        parent
                    } else {
                        shell_dirs.current.as_path()
                    }
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
            } else {
                new_current = Some(PathBuf::from(path));
            }
        }
    } else {
        println!("Please enter an appropriate path");
    }
    shell_dirs.update_dirs(new_current);

    //let exitStatus = ExitStatusExt::from_raw(0);
    //exitStatus
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cd_handles_relative_current() {
        let mut shell_dirs = ShellDirs::new();
        ShellDirs::setup(&mut shell_dirs);

        // Verify current relative
        let current = shell_dirs.current.clone();
        cd(&mut shell_dirs, Some("."));
        assert_eq!(shell_dirs.current, current);
        assert_eq!(shell_dirs.previous, current);

        let current = shell_dirs.current.clone();
        cd(&mut shell_dirs, Some("./"));
        assert_eq!(shell_dirs.current, current);
        assert_eq!(shell_dirs.previous, current);
    }

    #[test]
    fn cd_handles_relative_parent() {
        let mut shell_dirs = ShellDirs::new();
        ShellDirs::setup(&mut shell_dirs);

        let previous = shell_dirs.current.clone();
        cd(&mut shell_dirs, Some(".."));
        assert_eq!(shell_dirs.previous, previous);
        assert!(shell_dirs.current.as_path().eq(shell_dirs.previous.parent().unwrap()));
    }

    #[test]
    fn cd_handles_relative_parent_root() {
        let mut shell_dirs = ShellDirs::new();
        ShellDirs::setup(&mut shell_dirs);

        let mut path = "/";
        let previous = shell_dirs.current.clone();
        cd(&mut shell_dirs, Some(path));
        assert_eq!(shell_dirs.current, PathBuf::from(path));
        assert_eq!(shell_dirs.previous, previous);

        path = "..";
        cd(&mut shell_dirs, Some(path));
        assert_eq!(shell_dirs.current, PathBuf::from("/"));
        assert_eq!(shell_dirs.previous, PathBuf::from("/"));
    }

    #[test]
    fn cd_handles_relative_previous() {
        let mut shell_dirs = ShellDirs::new();
        ShellDirs::setup(&mut shell_dirs);

        // Verify previous
        let mut path = "-";
        let mut previous = shell_dirs.previous.clone();
        cd(&mut shell_dirs, Some(path));
        assert_eq!(shell_dirs.current, previous);

        path = "/tmp";
        previous = shell_dirs.current.clone();
        cd(&mut shell_dirs, Some(path));
        assert_eq!(shell_dirs.current, PathBuf::from(path));
        assert_eq!(shell_dirs.previous, previous);

        path = "-";
        previous = shell_dirs.previous.clone();
        cd(&mut shell_dirs, Some(path));
        assert_eq!(shell_dirs.current, previous);
        assert_eq!(shell_dirs.previous, PathBuf::from("/tmp"));
    }
/*
    fn cd_handles_absolute() {
    }

    fn cd_handles_root_parent() {
    }

    fn cd_handles_none_option() {
    }
    */
}
