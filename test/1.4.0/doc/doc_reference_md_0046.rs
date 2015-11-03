fn main() {
    struct Foo;
    
    trait Shape { fn area(&self) -> f64; }
    trait Circle : Shape { fn radius(&self) -> f64; }
    impl Shape for Foo {
        fn area(&self) -> f64 {
            0.0
        }
    }
    impl Circle for Foo {
        fn radius(&self) -> f64 {
            println!("calling area: {}", self.area());
    
            0.0
        }
    }
    
    let c = Foo;
    c.radius();
}
