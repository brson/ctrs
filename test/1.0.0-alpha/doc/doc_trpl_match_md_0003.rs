use std::cmp::Ordering;



fn cmp(a: i32, b: i32) -> Ordering {

    if a < b { Ordering::Less }

    else if a > b { Ordering::Greater }

    else { Ordering::Equal }

}



fn main() {

    let x = 5;

    let y = 10;



    match cmp(x, y) {

        Ordering::Less    => println!("less"),

        Ordering::Greater => println!("greater"),

        Ordering::Equal   => println!("equal"),

    }

}

