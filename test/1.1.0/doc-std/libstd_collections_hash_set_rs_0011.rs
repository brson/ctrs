fn main() {
    use std::collections::HashSet;
    let a: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<_> = [4, 2, 3, 4].iter().cloned().collect();
    
    // Can be seen as `a - b`.
    for x in a.difference(&b) {
        println!("{}", x); // Print 1
    }
    
    let diff: HashSet<_> = a.difference(&b).cloned().collect();
    assert_eq!(diff, [1].iter().cloned().collect());
    
    // Note that difference is not symmetric,
    // and `b - a` means something else:
    let diff: HashSet<_> = b.difference(&a).cloned().collect();
    assert_eq!(diff, [4].iter().cloned().collect());
}
