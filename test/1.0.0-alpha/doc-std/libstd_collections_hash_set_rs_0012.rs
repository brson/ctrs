fn main() {
    use std::collections::HashSet;
    let a: HashSet<int> = [1i, 2, 3].iter().map(|&x| x).collect();
    let b: HashSet<int> = [4i, 2, 3, 4].iter().map(|&x| x).collect();
    
    // Print 1, 4 in arbitrary order.
    for x in a.symmetric_difference(&b) {
        println!("{}", x);
    }
    
    let diff1: HashSet<int> = a.symmetric_difference(&b).map(|&x| x).collect();
    let diff2: HashSet<int> = b.symmetric_difference(&a).map(|&x| x).collect();
    
    assert_eq!(diff1, diff2);
    assert_eq!(diff1, [1i, 4].iter().map(|&x| x).collect());
}
