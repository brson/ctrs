fn main() {
    struct Circle {
    x: f64,
    y: f64,
    radius: f64,
    }
    
    impl Circle {
    fn reference(&self) {
    println!("taking self by reference!");
    }
    
    fn mutable_reference(&mut self) {
    println!("taking self by mutable reference!");
    }
    
    fn takes_ownership(self) {
    println!("taking ownership of self!");
    }
    }
}
