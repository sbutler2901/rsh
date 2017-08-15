use std::process::Command;

pub fn which(cmd: Option<&str>) {
    if let Some(cmd_unwrapped) = cmd {
        match cmd_unwrapped {
            "fg" | "bg" | "which" | "pushd" | "popd" | "dirs"
                => { println!("{} : shell builtin command", cmd_unwrapped); },
            _ => {
                if let Ok(mut child) = Command::new("/usr/bin/which")
                                                .arg(cmd_unwrapped)
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
}
