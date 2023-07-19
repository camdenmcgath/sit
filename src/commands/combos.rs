use crate::Args;
use std::process::Command;

pub fn commit(args: Args) -> Result<Vec<String>, &'static str> {
    vec![
        String::from("git add ."),
        format!(
            "git commit --message=\"{}\"",
            args.msg.unwrap_or(String::from("update"))
        ),
    ]
}

pub fn push() -> Result<Vec<String>, &'static str> {
    let branch = current_branch();
    vec![format!("git push --set-upstream origin {}", branch)]
}

pub fn update(args: Args) -> Result<Vec<String>, &'static str> {
    vec![commit(args.clone()), push()]
        .into_iter()
        .flatten()
        .collect()
}

pub fn make(args: Args) -> Result<Vec<String>, &'static str> {
    vec![
        String::from("git init"),
        format!(
            "git remote add origin {}",
            args.url
                .unwrap_or(String::from("url must be provided to add remote"))
        ),
    ]
}

pub fn get_combo(args: Args) -> Result<Vec<String>, &'static str> {
    match args.cmd.as_str() {
        "commit" => return commit(args),
        "push" => return push(),
        "update" => return update(args),
        "make" => {
            if !in_working_tree() {
                return make(args);
            } else {
                panic!("Already in a working tree!")
            }
        }
        _ => panic!("Invalid command"),
    };
}

pub fn current_branch() -> Result<String, &'static str> {
    if !in_working_tree() {
        Err("Not currently in a git repo or working tree.")
    }
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("git branch --show-current")
        .output()
        .expect("Failed to execute command");

    Ok(String::from_utf8_lossy(&output.stdout)
        .as_ref()
        .trim()
        .to_owned())
}

pub fn in_working_tree() -> bool {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("git rev-parse --is-inside-work-tree")
        .output()
        .expect("Failed to execute command");
    print!(
        "{}",
        String::from_utf8_lossy(&output.stdout).as_ref().trim()
    );
    if String::from_utf8_lossy(&output.stdout).as_ref().trim() == "true" {
        true
    } else {
        false
    }
}
