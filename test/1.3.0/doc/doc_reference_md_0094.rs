fn main() {
    type Pair<'a> = (i32, &'a str);
    let p: Pair<'static> = (10, "ten");
    let (a, b) = p;
    
    assert_eq!(a, 10);
    assert_eq!(b, "ten");
    assert_eq!(p.0, 10);
    assert_eq!(p.1, "ten");
}
