fn main() {
    use std::collections::HashSet;
    
    let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();
    
    let set = &a ^ &b;
    
    let mut i = 0;
    let expected = [1, 2, 4, 5];
    for x in set.iter() {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}
