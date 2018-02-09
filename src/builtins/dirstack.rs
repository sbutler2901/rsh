use std::path::PathBuf;
use shelldirs::ShellDirs;
use builtins::cd::cd;
//use utils::is_absolute_dir_path;
use dirpatherror::DirPathError;

/* Displays the directory stack */
pub fn dirs(pushed_dirs: &Vec<PathBuf>) {
    if !pushed_dirs.is_empty() {
        let pushed_reversed_iter = pushed_dirs.iter().rev();
        for dir in pushed_reversed_iter {
            print!("{} ", dir.display());
        }
        println!("");
    } else {
        println!("Directory stack is empty");
    }
}

/* Pushes the current dir onto the dir stack & changes to dir_path */
pub fn pushd(pushed_dirs: &mut Vec<PathBuf>, shell_dirs: &mut ShellDirs, dir_path: &PathBuf) -> Result<(), DirPathError> { 
        //is_absolute_dir_path(dir_path)?;
        cd(shell_dirs, dir_path)?;
        pushed_dirs.push(shell_dirs.previous.clone());
        dirs(&pushed_dirs);
        Ok(())
}

/* Pops the dir stack and changes to the returned directory */
pub fn popd(pushed_dirs: &mut Vec<PathBuf>, shell_dirs: &mut ShellDirs) -> Result<(), DirPathError> {
    if let Some(popped_dir) = pushed_dirs.pop() {
        cd(shell_dirs, &popped_dir)?;
        dirs(&pushed_dirs);
    } else {
        println!("popd: directory stack is empty");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dirs_handles_non_empty_stack() {
        let pushed_dirs: Vec<PathBuf> = Vec::new();
        pushed_dirs.push(PathBuf::from("/etc"));
        dirs(&pushed_dirs);
    }

    #[test]
    fn dirs_handles_empty_stack() {
        let pushed_dirs: Vec<PathBuf> = Vec::new();
        dirs(&pushed_dirs);
    }

    #[test]
    fn pushd_handles_relative_path() {
        let pushed_dirs: Vec<PathBuf> = Vec::new();
        let shell_dirs = ShellDirs::new();
        ShellDirs::setup(&mut shell_dirs);

        pushd(&mut pushed_dirs, &mut shell_dirs, &PathBuf::from("./"));
        pushd(&mut pushed_dirs, &mut shell_dirs, &PathBuf::from("../"));
        pushd(&mut pushed_dirs, &mut shell_dirs, &PathBuf::from("~/"));
        pushd(&mut pushed_dirs, &mut shell_dirs, &PathBuf::from("-/"));
    }

    #[test]
    fn pushd_handles_non_dir_path() {

    }

    #[test]
    fn pushd_handles_absolute_dir_path() {
    }
}
