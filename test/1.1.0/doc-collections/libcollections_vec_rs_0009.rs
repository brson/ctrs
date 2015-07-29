fn main() {
    let mut vec = Vec::with_capacity(10);
    
    // The vector contains no items, even though it has capacity for more
    assert_eq!(vec.len(), 0);
    
    // These are all done without reallocating...
    for i in 0..10 {
        vec.push(i);
    }
    
    // ...but this may make the vector reallocate
    vec.push(11);
}
