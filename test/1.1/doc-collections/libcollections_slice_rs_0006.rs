fn main() {
    let v = [10, 40, 30, 20, 50];
    let (v1, v2) = v.split_at(2);
    assert_eq!([10, 40], v1);
    assert_eq!([30, 20, 50], v2);
}
