use std::fmt::format;
use std::vec;

use crate::GitError;

use crate::commands::combos::*;
use crate::Args;

fn combo_test(args: Args, expected: Vec<String>) {
    let matched = get_combo(args.clone()).unwrap();
    let combo = match args.cmd.as_str() {
        "commit" => commit(args.clone()).unwrap(),
        "push" => push().unwrap(),
        "update" => update(args.clone()).unwrap(),
        "make" => make(args.clone()).unwrap(),
        _ => panic!("cmd not found"),
    };
    assert_eq!(
        matched, combo,
        "get_combo function failed to match {}",
        args.cmd
    );
    assert_eq!(
        combo, expected,
        "Did not get expected commit combo: got {:?}, expected {:?}",
        combo, expected
    );
}

#[test]
fn check_working_tree() {
    assert!(in_working_tree(), "Got {}", in_working_tree());
}
#[test]
fn check_current_branch() {
    assert_eq!(current_branch().unwrap(), "master")
}

#[test]
fn check_commit() {
    combo_test(
        Args {
            cmd: String::from("commit"),
            msg: Some(String::from("testing")),
            url: None,
        },
        vec![
            String::from("git add ."),
            String::from("git commit --message=\"testing\""),
        ],
    );
}

#[test]
fn check_push() {
    combo_test(
        Args {
            cmd: String::from("push"),
            msg: None,
            url: None,
        },
        vec![format!(
            "git push --set-upstream origin {}",
            current_branch().unwrap()
        )],
    );
}

#[test]
fn check_update() {
    combo_test(
        Args {
            cmd: String::from("update"),
            msg: Some(String::from("testing")),
            url: None,
        },
        vec![
            String::from("git add ."),
            String::from("git commit --message=\"testing\""),
            format!(
                "git push --set-upstream origin {}",
                current_branch().unwrap()
            ),
        ],
    );
}

//TODO: add test by running the CLI in the main dir
