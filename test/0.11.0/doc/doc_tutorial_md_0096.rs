fn main() {
    let square_explicit = |x: int| -> uint { (x * x) as uint };
    let square_infer    = |x: int|         { (x * x) as uint };
    
    println!("{}", square_explicit(20));  // 400
    println!("{}", square_infer(-20));    // 400
}