fn main() {
    let k = 10u;
    assert_eq!(Some(4u).unwrap_or_else(|| 2 * k), 4u);
    assert_eq!(None.unwrap_or_else(|| 2 * k), 20u);
}
