use std::fs;
use std::path::PathBuf;

// TODO - Create error to be handled in case it is not a dir path
pub fn is_dir_path(dir_path: &str) -> bool {
    if let Ok(metadata) = fs::metadata(dir_path) {
        metadata.is_dir()
    } else {
        let path_buf = PathBuf::from(dir_path);
        if path_buf.is_relative() {
            true
        } else {
            false
        }
    }
}
