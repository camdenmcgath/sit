use clap::Parser;
use std::io::{self, Write};
use std::process::Command;
use std::vec;

//TODO: GOAL: get basic functionality (git add . , commit, push)
//TODO: NEXT: organize (module for each combo), add support for other os/shells, pretty print
#[derive(Parser, Debug)]
struct Args {
    ///Spit command name mapping to a combo of git commands
    cmd: String,

    /// message to be passed to -m in git command
    msg: Option<String>,
}

fn get_sequence(cmd: String, msg: Option<String>) -> Result<Vec<&'static str>, &'static str> {
    match cmd.as_str() {
        "update" => {
            if let Some(message) = msg {
                Ok(vec![
                    "git add .",
                    format!("git commit -m \"{}\"", message).as_str(),
                    "git push origin master",
                ])
            } else {
                Ok(vec![
                    "git add .",
                    "git commit -m \"\"",
                    "git push origin master",
                ])
            }
        }
        _ => Err("Unknown command."),
    }
}
fn main() {
    let args = Args::parse();
    let sequence = get_sequence(args.cmd, args.msg).unwrap();
    //TODO: iterate through sequence and run each command (below code) each time
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(sequence)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        io::stdout().write_all(&output.stdout).unwrap();
    } else {
        io::stdout().write_all(&output.stdout).unwrap();
    }
}
