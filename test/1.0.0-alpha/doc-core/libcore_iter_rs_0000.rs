fn main() {
    let values = vec![1i, 2, 3];
    
    // "Syntactical sugar" taking advantage of an iterator
    for &x in values.iter() {
        println!("{}", x);
    }
    
    // Rough translation of the iteration without a `for` iterator.
    let mut it = values.iter();
    loop {
        match it.next() {
            Some(&x) => {
                println!("{}", x);
            }
            None => { break }
        }
    }
}
