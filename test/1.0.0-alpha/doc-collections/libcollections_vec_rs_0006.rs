fn main() {
    let mut stack = Vec::new();
    
    stack.push(1i);
    stack.push(2i);
    stack.push(3i);
    
    loop {
        let top = match stack.pop() {
            None => break, // empty
            Some(x) => x,
        };
        // Prints 3, 2, 1
        println!("{}", top);
    }
}
