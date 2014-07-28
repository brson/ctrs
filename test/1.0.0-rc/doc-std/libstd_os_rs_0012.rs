fn main() {
    use std::os;
    
    // Same as println!("{}", last_os_error());
    println!("{}", os::error_string(os::errno() as uint));
}
