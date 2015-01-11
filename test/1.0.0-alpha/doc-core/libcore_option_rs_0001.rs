fn main() {
    use std::boxed::Box;
    let optional: Option<Box<int>> = None;
    check_optional(&optional);
    
    let optional: Option<Box<int>> = Some(Box::new(9000));
    check_optional(&optional);
    
    fn check_optional(optional: &Option<Box<int>>) {
        match *optional {
            Some(ref p) => println!("have value {}", p),
            None => println!("have no value")
        }
    }
}
