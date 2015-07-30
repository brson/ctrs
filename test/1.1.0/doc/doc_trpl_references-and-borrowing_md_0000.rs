fn main() {
    fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    // do stuff with v1 and v2
    
    // hand back ownership, and the result of our function
    (v1, v2, 42)
    }
    
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    
    let (v1, v2, answer) = foo(v1, v2);
}
