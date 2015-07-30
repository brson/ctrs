fn main() {
    let optional: Option<Box<i32>> = None;
    check_optional(&optional);
    
    let optional: Option<Box<i32>> = Some(Box::new(9000));
    check_optional(&optional);
    
    fn check_optional(optional: &Option<Box<i32>>) {
    match *optional {
    Some(ref p) => println!("have value {}", p),
    None => println!("have no value")
    }
    }
}
