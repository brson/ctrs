fn main() {
    let s = "Hello".to_string();
    
    fn foo<T: AsRef<str>>(s: T) {
        let slice = s.as_ref();
    }
}
