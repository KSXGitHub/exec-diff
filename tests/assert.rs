use exec_diff::assert_eq_uni_diff;

#[test]
fn equal() {
    let value = "abc\ndef\nghi\n";
    assert_eq_uni_diff(value, value);
}

#[test]
#[should_panic(expected = "assertion failed: `(left == right)`")]
fn not_equal() {
    let left = "abc\ndef\nghi\n";
    let right = "abc\ndeF\nghi\njkl\n";
    assert_eq_uni_diff(left, right);
}
