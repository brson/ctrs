fn main() {
    // A stack-allocated array
    let array: [i32; 3] = [1, 2, 3];
    
    // A heap-allocated array
    let vector: Vec<i32> = vec![1, 2, 3];
    
    // A slice into an array
    let slice: &[i32] = &vector[..];
}
