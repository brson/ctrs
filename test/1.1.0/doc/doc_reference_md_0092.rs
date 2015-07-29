fn main() {
    type Pair<'a> = (i32, &'a str);

    let p: Pair<'static> = (10, "hello");

    let (a, b) = p;

    assert!(b != "world");

    assert!(p.0 == 10);

}
