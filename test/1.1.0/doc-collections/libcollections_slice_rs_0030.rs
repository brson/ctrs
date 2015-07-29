fn main() {
    let v = [10, 40, 30];    assert!(v.ends_with(&[30]));    assert!(v.ends_with(&[40, 30]));    assert!(!v.ends_with(&[50]));    assert!(!v.ends_with(&[50, 30]));}
