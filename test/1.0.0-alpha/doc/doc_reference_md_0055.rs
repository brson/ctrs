fn main() {
    use std::boxed::Box;

    trait Shape { }

    impl Shape for int { }

    let mycircle = 0i;

    let myshape: Box<Shape> = Box::new(mycircle) as Box<Shape>;

}
