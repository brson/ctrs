fn main() {
    use std::collections::HashSet;
    let a: HashSet<int> = [1i, 2, 3].iter().map(|&x| x).collect();
    let b: HashSet<int> = [4i, 2, 3, 4].iter().map(|&x| x).collect();
    
    // Can be seen as `a - b`.
    for x in a.difference(&b) {
        println!("{}", x); // Print 1
    }
    
    let diff: HashSet<int> = a.difference(&b).map(|&x| x).collect();
    assert_eq!(diff, [1i].iter().map(|&x| x).collect());
    
    // Note that difference is not symmetric,
    // and `b - a` means something else:
    let diff: HashSet<int> = b.difference(&a).map(|&x| x).collect();
    assert_eq!(diff, [4i].iter().map(|&x| x).collect());
}
