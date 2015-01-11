enum OptionalInt {

    Value(i32),

    Missing,

}



fn main() {

    let x = OptionalInt::Value(5);

    let y = OptionalInt::Missing;



    match x {

        OptionalInt::Value(n) => println!("x is {}", n),

        OptionalInt::Missing  => println!("x is missing!"),

    }



    match y {

        OptionalInt::Value(n) => println!("y is {}", n),

        OptionalInt::Missing  => println!("y is missing!"),

    }

}

