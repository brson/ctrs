fn main() {
    use std::collections::BTreeSet;
    
    let set: BTreeSet<usize> = [1, 2, 3, 4].iter().cloned().collect();
    
    for x in set.iter() {
    println!("{}", x);
    }
    
    let v: Vec<_> = set.iter().cloned().collect();
    assert_eq!(v, [1, 2, 3, 4]);
}
