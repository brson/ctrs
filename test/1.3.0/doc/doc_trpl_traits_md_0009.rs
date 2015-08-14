fn main() {
    trait HasArea {
        fn area(&self) -> f64;
    }
    
    impl HasArea for i32 {
        fn area(&self) -> f64 {
            println!("this is silly");
    
            *self as f64
        }
    }
    
    5.area();
}
