fn main() {
    let s = concat!("test", 10, 'b', true);
    assert_eq!(s, "test10btrue");
}
