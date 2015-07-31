fn main() {
    use std::path::Path;
    
    let string = String::from("foo.txt");
    let from_string = Path::new(&string);
    let from_path = Path::new(&from_string);
    assert_eq!(from_string, from_path);
}
