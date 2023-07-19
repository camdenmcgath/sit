use crate::Args;
use std::process::Command;

pub fn commit(args: Args) -> Vec<String> {
    vec![
        "git add .".to_string(),
        format!(
            "git commit --message=\"{}\"",
            args.msg.unwrap_or("update".to_string())
        ),
    ]
}

pub fn push() -> Vec<String> {
    let branch = current_branch();
    vec![format!("git push --set-upstream origin {}", branch)]
}

pub fn update(args: Args) -> Vec<String> {
    vec![commit(args.clone()), push()]
        .into_iter()
        .flatten()
        .collect()
}

pub fn add(args: &Args) -> Vec<String> {
    Vec::new()
}

pub fn get_combo(args: Args) -> Vec<String> {
    match args.cmd.as_str() {
        "commit" => return commit(args),
        "push" => return push(),
        "update" => return update(args),
        _ => panic!("Invalid command"),
    };
}

pub fn current_branch() -> String {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("git branch --show-current")
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout)
        .as_ref()
        .trim()
        .to_owned()
}
