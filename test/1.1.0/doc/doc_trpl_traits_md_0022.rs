fn main() {
    trait Foo {
        fn foo(&self);
    }
    trait FooBar : Foo {
        fn foobar(&self);
    }
    struct Baz;
    
    impl Foo for Baz {
    fn foo(&self) { println!("foo"); }
    }
    
    impl FooBar for Baz {
    fn foobar(&self) { println!("foobar"); }
    }
}
