fn main() {
    let v = [10, 40, 30];    assert!(v.starts_with(&[10]));    assert!(v.starts_with(&[10, 40]));    assert!(!v.starts_with(&[50]));    assert!(!v.starts_with(&[10, 50]));}
