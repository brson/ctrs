fn next_two(x: i32) -> (i32, i32) { (x + 1, x + 2) }



fn main() {

    let (x, y) = next_two(5);

    println!("x, y = {}, {}", x, y);

}

