use std::boxed::Box;

fn main() {

    let x = Box::new(5i);



    let y = add_one(x);



    println!("{}", y);

}



fn add_one(mut num: Box<int>) -> Box<int> {

    *num += 1;



    num

}

