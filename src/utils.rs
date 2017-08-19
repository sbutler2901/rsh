use std::path::PathBuf;
use shelldirs::ShellDirs;
use dirpatherror::DirPathError;

/// Takes a path and if it is relative it converts it to an absolute path
pub fn relative_to_absolute(shell_dirs: &ShellDirs, path: &PathBuf) -> PathBuf {
    let mut absolute: PathBuf = path.clone();
    if path.starts_with(".") {
        if path.eq(&PathBuf::from(".")) {
            absolute = shell_dirs.current.clone();
        } else if path.starts_with("./") {
            if let Ok(stripped) = path.strip_prefix("./") {
                absolute = shell_dirs.current.join(stripped);
            }
        }
    } else if path.starts_with("..") {
        let parent_path;
        if let Some(parent) = shell_dirs.current.parent() {
            parent_path = parent.to_path_buf();
        } else {
            parent_path = shell_dirs.current.clone();
        }

        if path.eq(&PathBuf::from("..")) {
            absolute = parent_path;
        } else if path.starts_with("../") {
            if let Ok(stripped) = path.strip_prefix("../") {
                absolute = parent_path.join(stripped);
            }
        }
    } else if path.starts_with("~") {
        if path.eq(&PathBuf::from("~")) {
            absolute = shell_dirs.user_home.clone();
        } else if path.starts_with("~/") {
            if let Ok(stripped) = path.strip_prefix("~/") {
                absolute = shell_dirs.user_home.join(stripped);
            }
        }
    } else if path.starts_with("-") {
        if path.eq(&PathBuf::from("-")) {
            absolute = shell_dirs.previous.clone();
        } else if path.starts_with("-/") {
            if let Ok(stripped) = path.strip_prefix("-") {
                absolute = shell_dirs.previous.join(stripped);
            }
        }
    }
    //println!("Absolute path is: {}", joined.display());
    absolute
}

// Only returns an Ok(bool) value if it is an absolute dir path. Else, it returns
// the corresponding DirPathError
pub fn is_dir_path(dir_path: &PathBuf) -> Result<bool, DirPathError> {
    let mut is_dir_path = Err(DirPathError::NotAbsolutePath);
    if dir_path.is_absolute() {
        if let Ok(metadata) = dir_path.metadata() {
            let is_dir_path = match metadata.is_dir() {
                true => Ok(true),
                false => Err(DirPathError::NotDirectoryPath),
            };
        }
    }
    is_dir_path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_dir_path_handle_relative() {
        // Current
        assert_eq!(is_dir_path(&PathBuf::from(".")), false);
        assert_eq!(is_dir_path(&PathBuf::from("./tmp")), false);
        assert_eq!(is_dir_path(&PathBuf::from("./tmp/test.txt")), false);

        // Previous
        assert_eq!(is_dir_path(&PathBuf::from("..")), false);
        assert_eq!(is_dir_path(&PathBuf::from("../tmp")), false);
        assert_eq!(is_dir_path(&PathBuf::from("../tmp/test.txt")), false);

        // User Home
        assert_eq!(is_dir_path(&PathBuf::from("~")), false);
        assert_eq!(is_dir_path(&PathBuf::from("~/tmp")), false);
        assert_eq!(is_dir_path(&PathBuf::from("~/tmp/test.txt")), false);

        // Previous
        assert_eq!(is_dir_path(&PathBuf::from("-")), false);
        assert_eq!(is_dir_path(&PathBuf::from("-/tmp")), false);
        assert_eq!(is_dir_path(&PathBuf::from("-/tmp/test.txt")), false);
    }

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

    #[test]
    fn relative_to_absolute_handle_relative_no_suffix() {
        let mut absolute_path;
        let mut shell_dirs = ShellDirs::new(); 
        ShellDirs::setup(&mut shell_dirs);
    
        // Current
        absolute_path = relative_to_absolute(&shell_dirs, &PathBuf::from("."));
        assert_eq!(absolute_path, shell_dirs.current);
       
        // User Home
        absolute_path = relative_to_absolute(&shell_dirs, &PathBuf::from("~"));
        assert_eq!(absolute_path, shell_dirs.user_home);

        // Previous
        absolute_path = relative_to_absolute(&shell_dirs, &PathBuf::from("-"));
        assert_eq!(absolute_path, shell_dirs.previous);

        // Parent - non root
        shell_dirs.current = PathBuf::from("/tmp");
        absolute_path = relative_to_absolute(&shell_dirs, &PathBuf::from(".."));
        assert_eq!(absolute_path, shell_dirs.current.parent().unwrap());

        // Parent - root
        shell_dirs.current = PathBuf::from("/");
        absolute_path = relative_to_absolute(&shell_dirs, &PathBuf::from(".."));
        assert_eq!(absolute_path, shell_dirs.current);
    }

    #[test]
    fn relative_to_absolute_handle_relative_with_suffix() {
        let mut absolute_path;
        let mut shell_dirs = ShellDirs::new();
        ShellDirs::setup(&mut shell_dirs);

        // Current
        absolute_path = relative_to_absolute(&shell_dirs, &PathBuf::from("./tmp"));
        assert_eq!(absolute_path, shell_dirs.current.join(PathBuf::from("tmp")));
        absolute_path = relative_to_absolute(&shell_dirs, &PathBuf::from("./tmp/test.txt"));
        assert_eq!(absolute_path, shell_dirs.current.join(PathBuf::from("tmp/test.txt")));

        // User Home
        absolute_path = relative_to_absolute(&shell_dirs, &PathBuf::from("~/tmp"));
        assert_eq!(absolute_path, shell_dirs.user_home.join(PathBuf::from("tmp")));
        absolute_path = relative_to_absolute(&shell_dirs, &PathBuf::from("~/tmp/test.txt"));
        assert_eq!(absolute_path, shell_dirs.user_home.join(PathBuf::from("tmp/test.txt")));

        // Previous
        absolute_path = relative_to_absolute(&shell_dirs, &PathBuf::from("-/tmp"));
        assert_eq!(absolute_path, shell_dirs.previous.join(PathBuf::from("tmp")));
        absolute_path = relative_to_absolute(&shell_dirs, &PathBuf::from("-/tmp/test.txt"));
        assert_eq!(absolute_path, shell_dirs.previous.join(PathBuf::from("tmp/test.txt")));

        // Parent - non root
        shell_dirs.current = PathBuf::from("/tmp");
        absolute_path = relative_to_absolute(&shell_dirs, &PathBuf::from("../tmp/test.txt"));
        assert_eq!(absolute_path, shell_dirs.current.parent().unwrap().join(PathBuf::from("tmp/test.txt")));

        // Parent - root
        shell_dirs.current = PathBuf::from("/");
        absolute_path = relative_to_absolute(&shell_dirs, &PathBuf::from("../tmp/test.txt"));
        assert_eq!(absolute_path, shell_dirs.current.join(PathBuf::from("tmp/test.txt")));
    }

    #[test]
    fn relative_to_absolute_handle_absolute() {
        let mut absolute_path;
        let mut shell_dirs = ShellDirs::new();
        ShellDirs::setup(&mut shell_dirs);

        absolute_path = relative_to_absolute(&shell_dirs, &PathBuf::from("/tmp"));
        assert_eq!(absolute_path, PathBuf::from("/tmp"));

        absolute_path = relative_to_absolute(&shell_dirs, &PathBuf::from("/tmp/test.txt"));
        assert_eq!(absolute_path, PathBuf::from("/tmp/test.txt"));
    }
}
