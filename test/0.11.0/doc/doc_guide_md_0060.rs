struct Point {
    x: int,
    y: int,
}

fn main() {
    let mut point = Point { x: 0i, y:  0i };

    point.x = 5;

    println!("The point is at ({}, {})", point.x, point.y);
}
