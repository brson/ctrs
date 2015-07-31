fn main() {
    let mut stack = Vec::new();
    
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while let Some(top) = stack.pop() {
    // Prints 3, 2, 1
    println!("{}", top);
    }
}
