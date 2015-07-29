fn main() {
    let result = std::f64::NAN.partial_cmp(&1.0);
    assert_eq!(result, None);
}
