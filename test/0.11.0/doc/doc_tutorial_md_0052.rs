fn main() {
    use std::mem::size_of; // bring `size_of` into the current scope, for convenience
    
    struct Foo {
        a: u32,
        b: u32,
        c: u32,
        d: u32
    }
    
    assert_eq!(size_of::<Foo>(), size_of::<u32>() * 4);
    
    struct Bar {
        a: Foo,
        b: Foo,
        c: Foo,
        d: Foo
    }
    
    assert_eq!(size_of::<Bar>(), size_of::<u32>() * 16);
}