fn main() {
    let four_lines = "foo\r\nbar\n\nbaz\n";
    let v: Vec<&str> = four_lines.lines().collect();
    
    assert_eq!(v, ["foo", "bar", "", "baz"]);
}
