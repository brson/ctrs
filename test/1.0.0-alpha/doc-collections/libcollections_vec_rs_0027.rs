fn main() {
    let mut v = Vec::new();
    assert!(v.is_empty());
    
    v.push(1i);
    assert!(!v.is_empty());
}
