fn main() {
    trait Num {

        fn from_int(n: int) -> Self;

    }

    impl Num for f64 {

        fn from_int(n: int) -> f64 { n as f64 }

    }

    let x: f64 = Num::from_int(42);

}
