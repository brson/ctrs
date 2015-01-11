fn main() {
    use std::collections::BTreeSet;
    
    let set: BTreeSet<uint> = [1u, 2, 3, 4].iter().map(|&x| x).collect();
    
    for x in set.iter() {
        println!("{}", x);
    }
    
    let v: Vec<uint> = set.iter().map(|&x| x).collect();
    assert_eq!(v, vec![1u,2,3,4]);
}
