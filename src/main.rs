mod cli;
mod commands;
use clap::Parser;
use cli::parser::Args;
use commands::combos::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, Write};
use std::process::Command;
use std::vec;

//TODO: GOAL: add more combos, logging, tests, publish
//TODO: NEXT: organize (module for each combo), add support for other os/shells, refine pretty print
//CONSIDER: anyhow for errors

fn main() {
    let args = Args::parse();
    let combo= match args.cmd.as_str() {
        "commit" => Combos::Commit(Commit::build(args)),
        "push" => Combos::Push(Push::build(args)),
        "update" => Combos::Update(Update::build(args)),
        _ => panic!("Invalid command"),
    }
    //let prog_bar = ProgressBar::new(sequence.len() as u64)
    //.with_style(ProgressStyle::with_template("{bar}  {pos}/{len} \n{msg}").unwrap());
    for cmd in combo.cmds.iter() {
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
