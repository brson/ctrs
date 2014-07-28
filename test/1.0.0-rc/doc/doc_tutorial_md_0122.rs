fn main() {
    type Circle = int; type Rectangle = bool;
    trait Drawable { fn draw(&self); }
    fn new_circle() -> Circle { 1 }
    fn new_rectangle() -> Rectangle { true }
    fn draw_all(shapes: &[Box<Drawable>]) {}
    
    impl Drawable for Circle { fn draw(&self) { /* ... */ } }
    impl Drawable for Rectangle { fn draw(&self) { /* ... */ } }
    
    let c: Box<Circle> = box new_circle();
    let r: Box<Rectangle> = box new_rectangle();
    draw_all([c as Box<Drawable>, r as Box<Drawable>]);
}