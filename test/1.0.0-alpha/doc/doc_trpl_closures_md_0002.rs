fn main() {

    let x: i32 = 5;



    let printer = |&:| { println!("x is: {}", x); };



    printer(); // prints "x is: 5"

}

