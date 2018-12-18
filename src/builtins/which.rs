use std::process::Command;

// TODO - When shell environment variable PATHS has been implemented this will need to be modified
pub fn which(cmd: &str) {
    match cmd {
        "bg" | "cd" | "dirs" | "fg" | "popd" | "pushd" | "pwd" | "which" | "jobs" | "alias"
            => { println!("{} : shell builtin command", cmd); },
        _ => {
            if let Ok(mut child) = Command::new("/usr/bin/which")
                                            .arg(cmd)
                                            .spawn()
            {
                if let Err(e) = child.wait() {
                    println!("Error waiting for child: {}", e);
                }
            } else {
                println!("Error executing /usr/bin/which");
            }
        }
    };
}
