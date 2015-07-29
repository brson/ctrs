fn main() {
    let input = b"Hello \xF0\x90\x80World";
    let output = String::from_utf8_lossy(input);
    assert_eq!(output, "Hello \u{FFFD}World");
}
