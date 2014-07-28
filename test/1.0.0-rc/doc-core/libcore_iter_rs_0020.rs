fn main() {
    let a = [1i, 2, 3, 4, 5];
    assert!(a.iter().last().unwrap() == &5);
}
