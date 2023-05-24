use exec_diff::{Color, Diff};
use pretty_assertions::assert_eq;

#[test]
fn color_always() {
    let left = "abc\ndef\nghi\n";
    let right = "abc\ndeF\nghi\njkl\n";
    let received = Diff::new(left, right)
        .color(Color::Always)
        .exec()
        .expect("execute diff")
        .expect("has diff");
    eprintln!("RECEIVED:\n{received}\n");
    let expected = include_str!("fixtures/color-always.txt");
    assert_eq!(received.trim_end(), expected.trim_end());
}

#[test]
fn color_never() {
    let left = "abc\ndef\nghi\n";
    let right = "abc\ndeF\nghi\njkl\n";
    let received = Diff::new(left, right)
        .color(Color::Never)
        .exec()
        .expect("execute diff")
        .expect("has diff");
    eprintln!("RECEIVED:\n{received}\n");
    let expected = include_str!("fixtures/color-never.txt");
    assert_eq!(received.trim_end(), expected.trim_end());
}

#[test]
fn non_unified() {
    let left = "abc\ndef\nghi\n";
    let right = "abc\ndeF\nghi\njkl\n";
    let received = Diff::new(left, right)
        .color(Color::Never)
        .unified(false)
        .exec()
        .expect("execute diff")
        .expect("has diff");
    eprintln!("RECEIVED:\n{received}\n");
    let expected = include_str!("fixtures/non-unified.txt");
    assert_eq!(received.trim_end(), expected.trim_end());
}

#[test]
fn no_diff() {
    let value = "abc\ndef\nghi\n";
    let received = Diff::new(value, value).exec().expect("execute diff");
    dbg!(&received);
    assert_eq!(received, None);
}
