fn main() {
    use std::collections::HashSet;
    let a: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<_> = [4, 2, 3, 4].iter().cloned().collect();
    
    // Print 1, 4 in arbitrary order.
    for x in a.symmetric_difference(&b) {
        println!("{}", x);
    }
    
    let diff1: HashSet<_> = a.symmetric_difference(&b).cloned().collect();
    let diff2: HashSet<_> = b.symmetric_difference(&a).cloned().collect();
    
    assert_eq!(diff1, diff2);
    assert_eq!(diff1, [1, 4].iter().cloned().collect());
}
