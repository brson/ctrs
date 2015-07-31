fn main() {
    use std::env;
    
    match env::home_dir() {
        Some(ref p) => println!("{}", p.display()),
        None => println!("Impossible to get your home dir!")
    }
}
