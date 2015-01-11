fn main() {
    struct Foo<T> { a: int, b: T }

    impl<T: PartialEq> PartialEq for Foo<T> {

        fn eq(&self, other: &Foo<T>) -> bool {

            self.a == other.a && self.b == other.b

        }

    

        fn ne(&self, other: &Foo<T>) -> bool {

            self.a != other.a || self.b != other.b

        }

    }

}
