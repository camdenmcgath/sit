use clap::Parser;
use std::io::{self, Write};
use std::process::Command;

#[derive(Parser, Debug)]
struct Args {
    ///Spit command name mapping to a combo of git commands
    cmd: String,

    /// message to be passed to -m in git command
    message: String,
}

fn main() {
    let args = Args::parse();

    let output = Command::new("powershell")
        .arg("-Command")
        .arg(args.cmd)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        io::stdout().write_all(&output.stdout).unwrap();
    } else {
        io::stdout().write_all(&output.stdout).unwrap();
    }
}
