fn main() {
    use std::env;
    
    // We assume that we are in a valid directory.
    let p = env::current_dir().unwrap();
    println!("The current directory is {}", p.display());
}
