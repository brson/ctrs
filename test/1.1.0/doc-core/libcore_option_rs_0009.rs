fn main() {
    let x = Some("value");
    assert_eq!(x.expect("the world is ending"), "value");
}
