fn main() {
    // Can't be stored on the stack directly
    struct Foo {
        info: u32,
        data: [u8],
    }
}
