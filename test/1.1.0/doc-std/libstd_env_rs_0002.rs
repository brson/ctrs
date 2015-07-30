fn main() {
    use std::env;
    
    // We will iterate through the references to the element returned by
    // env::vars();
    for (key, value) in env::vars() {
    println!("{}: {}", key, value);
    }
}
