fn main() {
    let v: Vec<char> = "abc åäö".chars().collect();
    assert_eq!(v, vec!['a', 'b', 'c', ' ', 'å', 'ä', 'ö']);
}
