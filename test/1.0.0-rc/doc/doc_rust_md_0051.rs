fn main() {
    trait Shape { }
    impl Shape for int { }
    let mycircle = 0i;
    let myshape: Box<Shape> = box mycircle as Box<Shape>;
}