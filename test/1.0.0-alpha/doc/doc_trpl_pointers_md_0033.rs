use std::boxed::Box;

fn add_one(x: &int) -> int {

    *x + 1

}



fn main() {

    let x = Box::new(5i);



    println!("{}", add_one(&*x));

}

