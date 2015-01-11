use std::cmp::Ordering;



fn cmp(a: i32, b: i32) -> Ordering {

    if a < b { Ordering::Less }

    else if a > b { Ordering::Greater }

    else { Ordering::Equal }

}



fn main() {

    let x = 5;

    let y = 10;



    let ordering = cmp(x, y);



    if ordering == Ordering::Less {

        println!("less");

    } else if ordering == Ordering::Greater {

        println!("greater");

    } else if ordering == Ordering::Equal {

        println!("equal");

    }

}

