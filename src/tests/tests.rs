use crate::commands::*;

#[cfg(tests)]
mod tests {
    #[test]
    fn check_current_branch() {
        assert_eq!(current_branch(), "master")
    }
}
