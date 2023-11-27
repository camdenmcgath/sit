mod commands;
mod tests;
mod runner;
use clap::Parser;
use commands::combos::*;

use std::io::{self, Write};


use crate::runner::PlatformRunner;
//TODO: GOAL: add more combos, logging, tests,
//TODO: NEXT: add support for other os/shells, refine pretty print
//TODO: add CI between github and crates, deploy downloadable binaries

#[derive(Parser, Debug, Clone)]
pub struct Args {
    ///Spit command name mapping to a combo of git commands
    pub cmd: String,

    /// message to be passed to -m in git command
    pub msg: Option<String>,

    /// optional url command
    pub url: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum GitError {
    #[error("Invalid command {0}")]
    InvalidCommand(String),
    #[error("In working tree, repo already initialized.")]
    AlreadyInit,
    #[error("Not in a working tree, current directory not a github repo.")]
    NotARepo,
    #[error("No message provided, but message is required for {0} command")]
    NoMessage(String),
    #[error("URL not provided but necessary for {0} command.")]
    NoURL(String),
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let combo = get_combo(args)?;
    //let prog_bar = ProgressBar::new(sequence.len() as u64)
    //.with_style(ProgressStyle::with_template("{bar}  {pos}/{len} \n{msg}").unwrap());
    for cmd in combo.into_iter() {
        if !in_working_tree() {
            return Err(GitError::NotARepo.into());
        }
        println!("\nRunning {}", cmd);
        println!("-----------------------------------------------");
      
        let output = Command::new("pwsh")
            .arg("-Command")
            .arg(&cmd)
            .output()
            .expect(format!("Failed to execute command in main {}", cmd).as_str());
        let output = PlatformRunner::for_platform()
            .execute(&cmd)
            .unwrap_or_else(|_| panic!("Failed to execute command in main {}", cmd));

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
        io::stdout().flush().unwrap();
    }
    //prog_bar.finish_with_message("Done");
    print!("\nFinished successfuly!");
    Ok(())
}
