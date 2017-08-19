use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DirPathError {
    NotAbsolutePath,
    NotDirectoryPath,
}

impl Error for DirPathError {
    fn description(&self) -> &str {
        match *self {
            DirPathError::NotAbsolutePath => {
                "The path provided was not absolute"
            },
            DirPathError::NotDirectoryPath => {
                "The path provided was not to a directory"
            },
        }
    }
}

impl fmt::Display for DirPathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DirPathError::NotAbsolutePath => {
                f.write_str("NotAbsolutePath")
            },
            DirPathError::NotDirectoryPath => {
                f.write_str("NotDirectoryPath")
            },
        }
    }
}

