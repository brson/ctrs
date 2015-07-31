fn main() {
    use std::io::Write;
    
    let mut w = Vec::new();
    write!(&mut w, "test").unwrap();
    write!(&mut w, "formatted {}", "arguments").unwrap();
}
