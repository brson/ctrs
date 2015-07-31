fn main() {
    for pair in "foo".chars().enumerate() {
    println!("{:?}", pair);
    }
    
    for pair in (0..).zip("foo".chars()) {
    println!("{:?}", pair);
    }
}
