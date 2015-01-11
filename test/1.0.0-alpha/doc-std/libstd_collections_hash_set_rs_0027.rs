fn main() {
    use std::collections::HashSet;
    
    let a: HashSet<int> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<int> = vec![3, 4, 5].into_iter().collect();
    
    let set: HashSet<int> = &a - &b;
    
    let mut i = 0;
    let expected = [1, 2];
    for x in set.iter() {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}
