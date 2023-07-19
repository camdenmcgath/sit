mod commands;
mod tests;
use clap::Parser;
use commands::combos::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, Write};
use std::process::Command;
//TODO: GOAL: add more combos, logging, tests, publish
//TODO: NEXT: organize (module for each combo), add support for other os/shells, refine pretty print
//CONSIDER: anyhow for errors

#[derive(Parser, Debug, Clone)]
pub struct Args {
    ///Spit command name mapping to a combo of git commands
    pub cmd: String,

    /// message to be passed to -m in git command
    pub msg: Option<String>,

    /// optional url command
    pub url: Option<String>,
}

fn main() {
    let args = Args::parse();
    let combo = get_combo(args);
    //let prog_bar = ProgressBar::new(sequence.len() as u64)
    //.with_style(ProgressStyle::with_template("{bar}  {pos}/{len} \n{msg}").unwrap());
    for cmd in combo {
        println!("\nRunning {}", cmd);
        println!("-----------------------------------------------");
        let output = Command::new("powershell")
            .arg("-Command")
            .arg(cmd)
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            //prog_bar.inc(1);
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        } else {
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        }
        io::stdout().flush().unwrap();
    }
    //prog_bar.finish_with_message("Done");
    print!("\nFinished successfuly!");
}
