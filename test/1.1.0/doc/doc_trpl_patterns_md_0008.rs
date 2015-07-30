fn main() {
    enum OptionalInt {
    Value(i32),
    Missing,
    }
    
    let x = OptionalInt::Value(5);
    
    match x {
    OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
    OptionalInt::Value(..) => println!("Got an int!"),
    OptionalInt::Missing => println!("No such luck."),
    }
}
