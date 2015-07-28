fn main() {
    use std::collections::HashSet;
    let a: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<_> = [4, 2, 3, 4].iter().cloned().collect();
    
    // Print 2, 3 in arbitrary order.
    for x in a.intersection(&b) {
        println!("{}", x);
    }
    
    let diff: HashSet<_> = a.intersection(&b).cloned().collect();
    assert_eq!(diff, [2, 3].iter().cloned().collect());
}
