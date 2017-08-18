use std::path::PathBuf;
use shelldirs::ShellDirs;

/// Takes a path and if it is relative it converts it to an absolute path
pub fn relative_to_absolute(shell_dirs: &ShellDirs, path: &PathBuf) -> PathBuf {
    let prefixed_path: PathBuf = path.clone();
    let mut joined: PathBuf = path.clone();
    if prefixed_path.starts_with("./") {
        if let Ok(stripped) = prefixed_path.strip_prefix("./") {
            joined = shell_dirs.current.join(stripped);
        }
    } else if prefixed_path.starts_with("../") {
        if let Ok(stripped) = prefixed_path.strip_prefix("../") {
            let parent_path;
            if let Some(parent) = shell_dirs.current.parent() {
                parent_path = parent.to_path_buf();
            } else {
                parent_path = shell_dirs.current.clone();
            }
            joined = parent_path.join(stripped);
        }
    } else if path.starts_with("~/") {
        if let Ok(stripped) = prefixed_path.strip_prefix("~/") {
            joined = shell_dirs.user_home.join(stripped);
        }
    } else if path.starts_with("-") {
        if let Ok(stripped) = prefixed_path.strip_prefix("-") {
            joined = shell_dirs.previous.join(stripped);
        }
    }
    //println!("Absolute path is: {}", joined.display());
    joined
}

// Assumes path is absolute else it returns false
pub fn is_dir_path(dir_path: &PathBuf) -> bool {
    let mut is_dir_path = false;
    if dir_path.is_absolute() {
        if let Ok(metadata) = dir_path.metadata() {
            is_dir_path = metadata.is_dir() 
        }
    }
    is_dir_path
}

#[cfg(test)]
mod tests {
    use super::*;

/*    #[test]
    fn is_dir_path_handle_relative() {
        assert_eq!(is_dir_path(&PathBuf::from("./tmp")), true);
        assert_eq!(is_dir_path(&PathBuf::from("~/opt")), true);
    }
*/
    #[test]
    fn is_dir_path_handle_absolute() {
        assert_eq!(is_dir_path(&PathBuf::from("/")), true);
        assert_eq!(is_dir_path(&PathBuf::from("/etc/ssh")), true);
    }

    #[test]
    fn is_dir_path_handle_file_name() {
        assert_eq!(is_dir_path(&PathBuf::from("/test.txt")), false);
        assert_eq!(is_dir_path(&PathBuf::from("/etc/test.txt")), false);
        assert_eq!(is_dir_path(&PathBuf::from("~/test.txt")), false);
    }
}
