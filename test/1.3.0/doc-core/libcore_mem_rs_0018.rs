fn main() {
    let mut v = vec![1, 2, 3];
    
    {
        let x = &v[0];
    
        drop(x); // this is now redundant, as `x` is going out of scope anyway
    }
    
    v.push(4); // no problems
}
