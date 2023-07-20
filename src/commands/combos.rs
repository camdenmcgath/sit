use crate::Args;
use crate::Error;
use std::process::Command;

pub fn commit(args: Args) -> Result<Vec<String>, Error> {
    if let Some(msg) = args.msg {
        Ok(vec![
            String::from("git add ."),
            format!("git commit --message=\"{}\"", msg),
        ])
    } else {
        Err(Error::NoMessage(String::from("commit")))
    }
}

pub fn push() -> Result<Vec<String>, Error> {
    let branch = current_branch()?;
    Ok(vec![format!("git push --set-upstream origin {}", branch)])
}

pub fn update(args: Args) -> Result<Vec<String>, Error> {
    Ok(vec![commit(args.clone())?, push()?]
        .into_iter()
        .flatten()
        .collect())
}

pub fn make(args: Args) -> Result<Vec<String>, Error> {
    if let Some(addr) = args.url {
        Ok(vec![
            String::from("git init"),
            format!("git remote add origin {}", addr),
        ])
    } else {
        Err(Error::NoURL(String::from("make")))
    }
}

pub fn get_combo(args: Args) -> Result<Vec<String>, Error> {
    match args.cmd.as_str() {
        "commit" => Ok(commit(args)?),
        "push" => Ok(push()?),
        "update" => Ok(update(args)?),
        "make" => {
            if !in_working_tree() {
                Ok(make(args)?)
            } else {
                Err(Error::AlreadyInit)
            }
        }
        _ => Err(Error::InvalidCommand(args.cmd)),
    }
}

pub fn current_branch() -> Result<String, Error> {
    if !in_working_tree() {
        Err(Error::NotARepo)
    } else {
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
}

pub fn in_working_tree() -> bool {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("git rev-parse --is-inside-work-tree")
        .output()
        .expect("Failed to execute command");
    if String::from_utf8_lossy(&output.stdout).as_ref().trim() == "true" {
        true
    } else {
        false
    }
}
