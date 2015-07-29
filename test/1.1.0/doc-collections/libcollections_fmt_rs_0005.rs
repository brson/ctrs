fn main() {
    let formatted_number = format!("{:.*}", 2, 1.234567);
    
    assert_eq!("1.23", formatted_number)
}
