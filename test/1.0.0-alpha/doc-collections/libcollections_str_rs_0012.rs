fn main() {
    let four_lines = "foo\nbar\n\nbaz\n";
    let v: Vec<&str> = four_lines.lines().collect();
    assert_eq!(v, vec!["foo", "bar", "", "baz"]);
}
