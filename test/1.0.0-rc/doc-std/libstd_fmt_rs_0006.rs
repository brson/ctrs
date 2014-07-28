fn main() {
    use std;

    mod fmt { pub type Result = (); }

    struct T;

    trait SomeName<T> {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result;

    }

}
