use crate::commands::combos::*;

#[test]
fn check_current_branch() {
    assert_eq!(current_branch(), "master")
}
