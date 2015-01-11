use std::cmp::Ordering;



fn cmp(a: i32, b: i32) -> Ordering {

    if a < b { Ordering::Less }

    else if a > b { Ordering::Greater }

    else { Ordering::Equal }

}



fn main() {

    let x = 5;

    let y = 10;



    println!("{}", match cmp(x, y) {

        Ordering::Less    => "less",

        Ordering::Greater => "greater",

        Ordering::Equal   => "equal",

    });

}

