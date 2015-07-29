fn main() {
    trait Container {

        type E;

        fn empty() -> Self;

        fn insert(&mut self, Self::E);

    }

    impl<T> Container for Vec<T> {

        type E = T;

        fn empty() -> Vec<T> { Vec::new() }

        fn insert(&mut self, x: T) { self.push(x); }

    }

}
