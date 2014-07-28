fn main() {
    type Circle = int; type Rectangle = int;
    trait Drawable { fn draw(&self); }
    impl Drawable for int { fn draw(&self) {} }
    fn new_circle() -> int { 1 }
    fn new_rectangle() -> int { 2 }
    // An owned object
    let owny: Box<Drawable> = box new_circle() as Box<Drawable>;
    // A borrowed object
    let stacky: &Drawable = &new_circle() as &Drawable;
}