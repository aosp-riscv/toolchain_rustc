use super::*;

#[test]
fn test_find_attr_val() {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    let s = r#"#[unstable(feature = "checked_duration_since", issue = "58402")]"#;
    assert_eq!(find_attr_val(s, "feature"), Some("checked_duration_since"));
=======
    let s = r#"#[unstable(feature = "tidy_test_never_used_anywhere_else", issue = "58402")]"#;
    assert_eq!(find_attr_val(s, "feature"), Some("tidy_test_never_used_anywhere_else"));
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    assert_eq!(find_attr_val(s, "issue"), Some("58402"));
    assert_eq!(find_attr_val(s, "since"), None);
}
