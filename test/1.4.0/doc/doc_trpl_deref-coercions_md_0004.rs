fn main() {
    struct Foo;
    
    impl Foo {
        fn foo(&self) { println!("Foo"); }
    }
    
    let f = &&Foo;
    
    f.foo();
}
