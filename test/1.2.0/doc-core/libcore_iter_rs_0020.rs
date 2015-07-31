fn main() {
    let mut it = 0..10;
    // sum the first five values
    let partial_sum = it.by_ref().take(5).fold(0, |a, b| a + b);
    assert_eq!(partial_sum, 10);
    assert_eq!(it.next(), Some(5));
}
