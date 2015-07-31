fn main() {
    let dish = ("Ham", "Eggs");
    
    // this body will be skipped because the pattern is refuted
    if let ("Bacon", b) = dish {
    println!("Bacon is served with {}", b);
    }
    
    // this body will execute
    if let ("Ham", b) = dish {
    println!("Ham is served with {}", b);
    }
}
