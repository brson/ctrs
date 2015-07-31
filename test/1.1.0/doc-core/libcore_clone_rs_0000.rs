fn main() {
    let hello = "Hello"; // &str implements Clone
    
    assert_eq!("Hello", hello.clone());
}
