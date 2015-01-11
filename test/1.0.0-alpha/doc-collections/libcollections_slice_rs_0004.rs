fn main() {
    let mut v = [5i, 4, 1, 3, 2];
    v.sort_by(|a, b| a.cmp(b));
    assert!(v == [1, 2, 3, 4, 5]);
    
    // reverse sorting
    v.sort_by(|a, b| b.cmp(a));
    assert!(v == [5, 4, 3, 2, 1]);
}
