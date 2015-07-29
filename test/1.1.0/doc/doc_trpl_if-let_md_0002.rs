fn main() {
    let option = Some(5);
    fn foo(x: i32) { }
    if let Some(x) = option {
        foo(x);
    }
}
