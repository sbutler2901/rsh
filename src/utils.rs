use std::fs;
use std::path::PathBuf;

// TODO - Create error to be handled in case it is not a dir path
pub fn is_dir_path(dir_path: &str) -> bool {
    println!("path: {}", dir_path);
    if let Ok(metadata) = fs::metadata(dir_path) {
        println!("metadata was okay");
        metadata.is_dir()
    } else {
        false
        /*let path_buf = PathBuf::from(dir_path);
        if path_buf.is_relative() {
            println!("is a relative path");
            true
        } else {
            println!("is not a relative path");
            false
        }*/
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
