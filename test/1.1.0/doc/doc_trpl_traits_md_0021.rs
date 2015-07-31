fn main() {
    trait Foo {
        fn foo(&self);
    }
    
    trait FooBar : Foo {
        fn foobar(&self);
    }
}
