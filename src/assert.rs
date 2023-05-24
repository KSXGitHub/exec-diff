use super::Diff;

/// Assert that two strings are equal.
///
/// It will fallback to [`pretty_assertions`] if `diff` was failed to execute.
pub fn assert_eq_uni_diff<Left, Right>(left: Left, right: Right)
where
    Left: AsRef<str>,
    Right: AsRef<str>,
{
    Diff::new(left, right).assert_eq()
}
