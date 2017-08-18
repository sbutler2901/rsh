//use std::fs;
use std::path::PathBuf;
use shelldirs::ShellDirs;


pub fn relative_to_absolute(shell_dirs: &ShellDirs, dir_path: &mut PathBuf) {
    let prefixed_path = dir_path.clone();
    if prefixed_path.starts_with("./") {
        if let Ok(stripped) = prefixed_path.strip_prefix("./") {
            //let prefix: PathBuf = shell_dirs.current.clone();
            let joined = shell_dirs.current.join(stripped);
            dir_path.clone_from(&joined);
        }
    } else if prefixed_path.starts_with("../") {
        if let Ok(stripped) = prefixed_path.strip_prefix("../") {
            let parent_path;
            if let Some(parent) = shell_dirs.current.parent() {
                parent_path = parent.to_path_buf();
            } else {
                parent_path = shell_dirs.current.clone();
            }
            let joined = parent_path.join(stripped);
            dir_path.clone_from(&joined);
        }
    } else if dir_path.starts_with("~/") {
        if let Ok(stripped) = prefixed_path.strip_prefix("~/") {
            //let prefix: PathBuf = shell_dirs.user_home.clone();
            let joined = shell_dirs.user_home.join(stripped);
            dir_path.clone_from(&joined);
        }
    } else if dir_path.starts_with("-") {
        if let Ok(stripped) = prefixed_path.strip_prefix("-") {
            //let prefix: PathBuf = shell_dirs.user_home.clone();
            let joined = shell_dirs.previous.join(stripped);
            dir_path.clone_from(&joined);
        }
    }
    println!("Absolute path is: {}", dir_path.display());
}

// Assumes dir_path is an absolute path
pub fn is_dir_path(dir_path: &PathBuf) -> bool {
    if let Ok(metadata) = dir_path.metadata() {
        metadata.is_dir() 
    } else {
        false
    }
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
