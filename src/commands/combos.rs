use crate::cli::parser::Args;
use enum_dispatch::enum_dispatch;

pub trait Combo {
    fn build(args: Args) -> Vec<String>;
}

pub struct Commit {
    cmds: Vec<String>,
}

impl Combo for Commit {
    fn build(args: Args) -> Vec<String> {
        vec![
            "git add .".to_string(),
            format!("commit -m {}", args.msg.unwrap_or_default()),
        ]
    }
}

pub struct Push {
    cmds: Vec<String>,
}

impl Combo for Push {
    fn build(args: Args) -> Vec<String> {
        vec!["git push origin master".to_string()]
    }
}

pub struct Update {
    cmds: Vec<String>,
}

impl Combo for Update {
    fn build(args: Args) -> Vec<String> {
        Vec::new() // add vector to pipe things wtih
    }
}
pub struct AddEmpty {
    cmds: Vec<String>,
}

impl Combo for AddEmpty {
    fn build(args: Args) -> Vec<String> {
        Vec::new()
    }
}
