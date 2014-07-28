fn main() {
    fn some_condition() -> bool { true }
    struct Foo { f: int }
    fn example3() -> int {
        let mut x = box Foo {f: 3};
        if some_condition() {
            let y = &x.f;      // -+ L
            return *y;         //  |
        }                      // -+
        x = box Foo {f: 4};
        // ...
    return 0;
    }
}