fn main() {
    use std::env;
    
    match env::current_exe() {
        Ok(exe_path) => println!("Path of this executable is: {}",
                                  exe_path.display()),
        Err(e) => println!("failed to get current exe path: {}", e),
    };
}
