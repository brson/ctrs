fn main() {
    use std::collections::BTreeSet;
    
    let a: BTreeSet<int> = vec![1, 2, 3].into_iter().collect();
    let b: BTreeSet<int> = vec![2, 3, 4].into_iter().collect();
    
    let result: BTreeSet<int> = &a ^ &b;
    let result_vec: Vec<int> = result.into_iter().collect();
    assert_eq!(result_vec, vec![1, 4]);
}
