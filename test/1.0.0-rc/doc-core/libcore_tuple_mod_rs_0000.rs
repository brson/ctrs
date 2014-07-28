fn main() {
    let pair = ("pi", 3.14f64);
    assert_eq!(pair.val0(), "pi");
    assert_eq!(pair.val1(), 3.14f64);
}
