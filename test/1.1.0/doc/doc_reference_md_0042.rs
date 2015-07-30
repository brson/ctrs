fn main() {
    trait Num {
    fn from_i32(n: i32) -> Self;
    }
    impl Num for f64 {
    fn from_i32(n: i32) -> f64 { n as f64 }
    }
    let x: f64 = Num::from_i32(42);
}
