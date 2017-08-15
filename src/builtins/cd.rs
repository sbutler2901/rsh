use std::path::PathBuf;
use std::env;

use shelldirs::ShellDirs;
use utils::is_dir_path;

pub fn cd(shell_dirs: &mut ShellDirs, path_wrapped: Option<&str>) /*-> ExitStatus*/ {
    if let Some(path_unwrapped) = path_wrapped {
        if is_dir_path(path_unwrapped) {
            let temp_path_buf: PathBuf;
            let path = match path_unwrapped {
                "." => { shell_dirs.current.as_path() },
                ".." => { 
                    // FIXME - panics when this is provided and in the root directory
                    shell_dirs.current.parent().unwrap() 
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
            }
        }
    } else {
        println!("Please enter an appropriate path");
    }
    shell_dirs.update_dirs();

    //let exitStatus = ExitStatusExt::from_raw(0);
    //exitStatus
}
