fn main() {
    let xs = [-3i, 0, 1, 5, -10];
    assert_eq!(*xs.iter().max_by(|x| x.abs()).unwrap(), -10);
}
