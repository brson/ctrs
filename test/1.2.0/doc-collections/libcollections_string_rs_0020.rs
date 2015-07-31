fn main() {
    let mut v = String::new();
    assert!(v.is_empty());
    v.push('a');
    assert!(!v.is_empty());
}
