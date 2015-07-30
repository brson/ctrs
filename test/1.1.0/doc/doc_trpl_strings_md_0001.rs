fn main() {
    let mut s = "Hello".to_string(); // mut s: String
    println!("{}", s);
    
    s.push_str(", world.");
    println!("{}", s);
}
