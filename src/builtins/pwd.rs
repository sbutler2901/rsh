use shelldirs::ShellDirs;

pub fn pwd(shell_dirs: &ShellDirs) {
    println!("{}", shell_dirs.current.display());
}
