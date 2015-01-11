fn main() {
    #![allow(non_shorthand_field_patterns)]

    struct Point {

        x: int,

        y: int,

    }

    

    let origin = Point { x: 0i, y: 0i };

    

    match origin {

        Point { x: x, y: y } => println!("({},{})", x, y),

    }

}
