fn main() {
    struct Point;
    use std::ops::Add;
    impl Add<i32> for Point {
        type Output = f64;
    
        fn add(self, rhs: i32) -> f64 {
            // add an i32 to a Point and get an f64
    1.0
        }
    }
}
