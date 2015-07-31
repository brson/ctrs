fn main() {
    let mut s = String::from("foo");
    assert_eq!(s.remove(0), 'f');
    assert_eq!(s.remove(1), 'o');
    assert_eq!(s.remove(0), 'o');
}
