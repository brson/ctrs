fn main() {
    use std::collections::HashSet;
    
    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![2, 3, 4].into_iter().collect();
    
    let set = &a & &b;
    
    let mut i = 0;
    let expected = [2, 3];
    for x in &set {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}
