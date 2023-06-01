use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, Write};
use std::process::Command;
use std::vec;

//TODO: GOAL: add more combos, logging, tests, publish
//TODO: NEXT: organize (module for each combo), add support for other os/shells, pretty print
//CONSIDER: anyhow for errors, indicatif for printing progress
#[derive(Parser, Debug)]
struct Args {
    ///Spit command name mapping to a combo of git commands
    cmd: String,

    /// message to be passed to -m in git command
    msg: Option<String>,
}

fn get_sequence(cmd: String, msg: Option<String>) -> Result<Vec<String>, &'static str> {
    match cmd.as_str() {
        "update" => {
            if let Some(message) = msg {
                //TODO: macro to make this conciser? https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=bfb5144953af92f8ff8a2ec9b5861c93
                Ok(vec![
                    String::from("git add ."),
                    format!("git commit -m \"{}\"", message),
                    String::from("git push origin master"),
                ])
            } else {
                Ok(vec![
                    String::from("git add ."),
                    String::from("git commit -m \" \""),
                    String::from("git push origin master"),
                ])
            }
        }
        _ => Err("Unknown command."),
    }
}
fn main() {
    let args = Args::parse();
    let sequence = get_sequence(args.cmd, args.msg).unwrap();
    let prog_bar = ProgressBar::new(sequence.len() as u64)
        .with_style(ProgressStyle::with_template("{bar:50}{msg}").unwrap());
    for cmd in sequence.iter() {
        let output = Command::new("powershell")
            .arg("-Command")
            .arg(cmd)
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            prog_bar.inc(1);
            io::stdout().write_all(&output.stdout).unwrap();
        } else {
            io::stdout().write_all(&output.stdout).unwrap();
        }
    }
    prog_bar.finish_with_message("Done");
}
