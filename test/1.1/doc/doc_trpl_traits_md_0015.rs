fn main() {
    use std::fmt::Debug;

    

    fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {

        x.clone();

        y.clone();

        println!("{:?}", y);

    }

}
