use std::boxed::Box;

fn main() {

    let x = Box::new(5i);



    add_one(x);

}



fn add_one(mut num: Box<int>) {

    *num += 1;

}

