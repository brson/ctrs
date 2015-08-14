fn main() {
    trait Foo {
        fn bar(&self);
        fn baz(&self) { println!("We called baz."); }
    }
}
