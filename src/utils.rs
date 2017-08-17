//use std::fs;
use std::path::PathBuf;

// TODO - Create error to be handled in case it is not a dir path
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

    #[test]
    fn is_dir_path_handle_relative() {
        assert_eq!(is_dir_path("./tmp"), true);
        assert_eq!(is_dir_path("~/opt"), true);
    }

    #[test]
    fn is_dir_path_handle_absolute() {
        assert_eq!(is_dir_path("/"), true);
        assert_eq!(is_dir_path("/etc/ssh"), true);
    }

    #[test]
    fn is_dir_path_handle_file_name() {
        assert_eq!(is_dir_path("/test.txt"), false);
        assert_eq!(is_dir_path("/etc/test.txt"), false);
        assert_eq!(is_dir_path("~/test.txt"), false);
    }
}
