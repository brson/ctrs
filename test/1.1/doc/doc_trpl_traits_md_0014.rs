fn main() {
    use std::fmt::Debug;

    

    fn foo<T: Clone + Debug>(x: T) {

        x.clone();

        println!("{:?}", x);

    }

}
