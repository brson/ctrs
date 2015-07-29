fn main() {
    enum OptionalInt {
        Value(i32),
        Missing,
    }
    
    let x = OptionalInt::Value(5);
    
    match x {
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck."),
    }
}
