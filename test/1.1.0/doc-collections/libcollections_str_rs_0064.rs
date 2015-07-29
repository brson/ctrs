fn main() {
    let s = " Hello\tworld\t";
    assert_eq!(s.trim_left(), "Hello\tworld\t");
}
