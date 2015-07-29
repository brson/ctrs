fn main() {
    assert_eq!(format!("{} {:?}", 3, 4), "3 4");    assert_eq!(format!("{} {:?}", 'a', 'b'), "a 'b'");    assert_eq!(format!("{} {:?}", "foo\n", "bar\n"), "foo\n \"bar\\n\"");}
