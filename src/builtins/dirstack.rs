use std::path::PathBuf;
use shelldirs::ShellDirs;
use builtins::cd::cd;

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

pub fn pushd(pushed_dirs: &mut Vec<PathBuf>, shell_dirs: &mut ShellDirs, path: &PathBuf) {
    pushed_dirs.push(PathBuf::from(shell_dirs.current.as_path()));
    cd(shell_dirs, path);
    dirs(&pushed_dirs);
   /* } else {
            println!("pushd: {} is not a directory", dir_path);
        }
    }*/
}

pub fn popd(pushed_dirs: &mut Vec<PathBuf>, shell_dirs: &mut ShellDirs) {
    if let Some(popped_dir) = pushed_dirs.pop() {
        cd(shell_dirs, &popped_dir);
        dirs(&pushed_dirs);
    } else {
        println!("popd: directory stack is empty");
    }
}


