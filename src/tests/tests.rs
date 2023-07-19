use std::fmt::format;
use std::vec;

use crate::commands::combos::*;
use crate::Args;

fn combo_test(args: Args, expected: Vec<String>) {
    let matched = get_combo(args.clone())?;
    let combo = get_combo(args.clone())?;
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
    assert_eq!(current_branch(), "master")
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
            current_branch()
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
            format!("git push --set-upstream origin {}", current_branch()),
        ],
    );
}

//TODO: add test by running the CLI in the main dir
