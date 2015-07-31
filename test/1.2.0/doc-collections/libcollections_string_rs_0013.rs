fn main() {
    let mut s = String::from("abc");
    s.push('1');
    s.push('2');
    s.push('3');
    assert_eq!(s, "abc123");
}
