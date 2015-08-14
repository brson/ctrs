fn main() {
    struct Foo; // No fields = no size
    
    // All fields have no size = no size
    struct Baz {
        foo: Foo,
        qux: (),      // empty tuple has no size
        baz: [u8; 0], // empty array has no size
    }
}
