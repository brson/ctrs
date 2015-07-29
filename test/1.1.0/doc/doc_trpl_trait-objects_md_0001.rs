fn main() {
    trait Foo { fn method(&self) -> String; }
    impl Foo for u8 {
        fn method(&self) -> String { format!("u8: {}", *self) }
    }
    
    impl Foo for String {
        fn method(&self) -> String { format!("string: {}", *self) }
    }
}
