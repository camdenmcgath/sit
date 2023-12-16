use crate::runner::PlatformRunner;
use crate::Args;
use crate::GitError;

pub fn commit(args: Args) -> Result<Vec<String>, GitError> {
    if let Some(msg) = args.msg {
        Ok(vec![
            String::from("git add ."),
            format!("git commit --message=\"{}\"", msg),
        ])
    } else {
        Err(GitError::NoMessage(String::from("commit")))
    }
}

pub fn push() -> Result<Vec<String>, GitError> {
    let branch = current_branch()?;
    Ok(vec![format!("git push --set-upstream origin {}", branch)])
}

pub fn update(args: Args) -> Result<Vec<String>, GitError> {
    Ok(vec![commit(args.clone())?, push()?]
        .into_iter()
        .flatten()
        .collect())
}

pub fn make(args: Args) -> Result<Vec<String>, GitError> {
    if let Some(addr) = args.url {
        Ok(vec![
            String::from("git init"),
            format!("git remote add origin {}", addr),
        ])
    } else {
        Err(GitError::NoURL(String::from("make")))
    }
}

pub fn get_combo(args: Args) -> Result<Vec<String>, GitError> {
    match args.cmd.as_str() {
        "commit" => Ok(commit(args)?),
        "push" => Ok(push()?),
        "update" => Ok(update(args)?),
        "make" => {
            if !in_working_tree() {
                Ok(make(args)?)
            } else {
                Err(GitError::AlreadyInit)
            }
        }
        _ => Err(GitError::InvalidCommand(args.cmd)),
    }
}

pub fn current_branch() -> Result<String, GitError> {
    if !in_working_tree() {
        Err(GitError::NotARepo)
    } else {
        let output = PlatformRunner::for_platform()
            .execute("git branch --show-current")
            .expect("Failed to execute command in current_branch()");

        Ok(String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .trim()
            .to_owned())
    }
}

// need to check the git rev-parse function
pub fn in_working_tree() -> bool {
    let output = PlatformRunner::for_platform()
        .execute("git rev-parse --is-inside-work-tree")
        .expect("Failed to execute command in in_working_tree");
    String::from_utf8_lossy(&output.stdout).as_ref().trim() == "true"
}
