fn main() {
    use std::env;
    
    // We will iterate through the references to the element returned by
    // env::vars_os();
    for (key, value) in env::vars_os() {
        println!("{:?}: {:?}", key, value);
    }
}
