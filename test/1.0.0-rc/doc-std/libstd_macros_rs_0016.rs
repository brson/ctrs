fn main() {
    let s = concat!("test", 10i, 'b', true);
    assert_eq!(s, "test10btrue");
}
