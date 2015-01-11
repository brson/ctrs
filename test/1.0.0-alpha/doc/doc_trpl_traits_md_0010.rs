fn main() {
    trait HasArea {

        fn area(&self) -> f64;

    }

    

    impl HasArea for int {

        fn area(&self) -> f64 {

            println!("this is silly");

    

            *self as f64

        }

    }

    

    5i.area();

}
