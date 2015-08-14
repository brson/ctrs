fn main() {
    use std::cmp::Ordering;
    struct MyType;
    unsafe trait UnsafeOrd { fn cmp(&self, other: &Self) -> Ordering; }
    unsafe impl UnsafeOrd for MyType {
        fn cmp(&self, other: &Self) -> Ordering {
            Ordering::Equal
        }
    }
}
