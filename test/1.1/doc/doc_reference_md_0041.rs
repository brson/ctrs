fn main() {
    trait Shape { }

    impl Shape for i32 { }

    let mycircle = 0i32;

    let myshape: Box<Shape> = Box::new(mycircle) as Box<Shape>;

}
