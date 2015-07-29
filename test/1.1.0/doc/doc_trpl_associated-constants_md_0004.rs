fn main() {
    #![feature(associated_consts)]
    
    struct Foo;
    
    impl Foo {
        pub const FOO: u32 = 3;
    }
}
