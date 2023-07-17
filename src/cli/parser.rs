use clap::Parser;
#[derive(Parser, Debug)]
pub struct Args {
    ///Spit command name mapping to a combo of git commands
    pub cmd: String,

    /// message to be passed to -m in git command
    pub msg: Option<String>,

    /// optional url command
    pub url: Option<String>,
}
