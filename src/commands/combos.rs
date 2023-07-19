use crate::Args;

pub fn commit(args: Args) -> Vec<String> {
    vec![
        "git add .".to_string(),
        format!("git commit -m {}", args.msg.unwrap_or("update".to_string())),
    ]
}

pub fn push(args: Args) -> Vec<String> {
    vec!["git push origin master".to_string()]
}

pub fn update(args: Args) -> Vec<String> {
    vec![commit(args.clone()), push(args.clone())]
        .into_iter()
        .flatten()
        .collect()
}

pub fn add(args: &Args) -> Vec<String> {
    Vec::new()
}
