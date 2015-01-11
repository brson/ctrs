fn main() {
    enum OptionalInt {

        Value(int),

        Missing,

    }

    

    let x = OptionalInt::Value(5i);

    

    match x {

        OptionalInt::Value(..) => println!("Got an int!"),

        OptionalInt::Missing   => println!("No such luck."),

    }

}
