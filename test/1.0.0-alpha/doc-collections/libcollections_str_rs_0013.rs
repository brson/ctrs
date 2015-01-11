fn main() {
    let four_lines = "foo\r\nbar\n\r\nbaz\n";
    let v: Vec<&str> = four_lines.lines_any().collect();
    assert_eq!(v, vec!["foo", "bar", "", "baz"]);
}
