use crate::Args;

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
    vec!["git push origin master".to_string()]
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
