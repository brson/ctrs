fn main() {
    assert_eq!(format!("{} {:?}", 3i32, 4i32), "3 4i32");
    assert_eq!(format!("{} {:?}", 'a', 'b'), "a 'b'");
    assert_eq!(format!("{} {:?}", "foo\n", "bar\n"), "foo\n \"bar\\n\"");
}
