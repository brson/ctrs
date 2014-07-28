fn main() {
    type Pair<'a> = (int, &'a str);
    let p: Pair<'static> = (10, "hello");
    let (a, b) = p;
    assert!(b != "world");
}