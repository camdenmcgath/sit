use crate::commands::combos::*;
use crate::Args;

#[test]
fn check_current_branch() {
    assert_eq!(current_branch(), "master")
}
#[test]
fn check_combo_match() {
    let commit_args = Args {
        cmd: String::from("commit"),
        msg: Some(String::from("testing")),
        url: None,
    };
    assert_eq!(get_combo(commit_args.clone()), commit(commit_args));

    let push_args = Args {
        cmd: String::from("push"),
        msg: None,
        url: None,
    };
    assert_eq!(get_combo(push_args), push());

    //TODO: finish tests
    //TODO: add test by running the CLI in the main dir
}
