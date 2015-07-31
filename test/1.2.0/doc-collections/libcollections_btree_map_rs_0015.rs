fn main() {
    use std::collections::BTreeMap;
    
    let mut count: BTreeMap<&str, usize> = BTreeMap::new();
    
    // count the number of occurrences of letters in the vec
    for x in vec!["a","b","a","c","a","b"] {
    *count.entry(x).or_insert(0) += 1;
    }
    
    assert_eq!(count["a"], 3);
}
