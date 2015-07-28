fn main() {
    trait Container {

        type E;

        fn empty() -> Self;

        fn insert(&mut self, Self::E);

    }

}
