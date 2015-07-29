fn main() {
    fn foo() {
        ...
    }
    
    fn frobnicate(a: Bar, b: Bar,
                  c: Bar, d: Bar)
                  -> Bar {
        ...
    }
    
    trait Bar {
        fn baz(&self);
    }
    
    impl Bar for Baz {
        fn baz(&self) {
            ...
        }
    }
    
    frob(|x| {
        x.transpose()
    })
}
