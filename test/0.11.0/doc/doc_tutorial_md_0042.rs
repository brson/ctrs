fn main() {
    struct Inches(int);
    let length_with_unit = Inches(10);
    let Inches(integer_length) = length_with_unit;
    println!("length is {} inches", integer_length);
}