fn main() {
    let four_lines = "foo\r\nbar\n\r\nbaz";
    let v: Vec<&str> = four_lines.lines_any().collect();
    
    assert_eq!(v, ["foo", "bar", "", "baz"]);
}
