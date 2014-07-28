fn main() {
    struct Inches(int);
    struct Centimeters(int);
    
    let length = Inches(10);
    
    let Inches(integer_length) = length;
    println!("length is {} inches", integer_length);
}