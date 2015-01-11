fn main() {
    use std::collections::HashSet;
    let a: HashSet<int> = [1i, 2, 3].iter().map(|&x| x).collect();
    let b: HashSet<int> = [4i, 2, 3, 4].iter().map(|&x| x).collect();
    
    // Print 1, 2, 3, 4 in arbitrary order.
    for x in a.union(&b) {
        println!("{}", x);
    }
    
    let diff: HashSet<int> = a.union(&b).map(|&x| x).collect();
    assert_eq!(diff, [1i, 2, 3, 4].iter().map(|&x| x).collect());
}
