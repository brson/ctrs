fn main() {
    trait Changer {
        fn change(mut self) -> Self;
        fn modify(mut self: Box<Self>) -> Box<Self>;
    }
}
