fn main() {
    let mut x = Some(2u);
    match x.as_mut() {
        Some(v) => *v = 42,
        None => {},
    }
    assert_eq!(x, Some(42u));
}
