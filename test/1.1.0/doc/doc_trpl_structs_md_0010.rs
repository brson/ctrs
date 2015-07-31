fn main() {
    struct Inches(i32);
    
    let length = Inches(10);
    
    let Inches(integer_length) = length;
    println!("length is {} inches", integer_length);
}
