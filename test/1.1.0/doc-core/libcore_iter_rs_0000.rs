fn main() {
    let values = vec![1, 2, 3];
    
    for x in values {
        println!("{}", x);
    }
    
    // Rough translation of the iteration without a `for` iterator.
    let values = vec![1, 2, 3];
    let mut it = values.into_iter();
    loop {
        match it.next() {
            Some(x) => println!("{}", x),
            None => break,
        }
    }
}
