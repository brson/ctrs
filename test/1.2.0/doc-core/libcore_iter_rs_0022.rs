fn main() {
    let vec = vec![1, 2, 3, 4];
    let (even, odd): (Vec<_>, Vec<_>) = vec.into_iter().partition(|&n| n % 2 == 0);
    assert_eq!(even, [2, 4]);
    assert_eq!(odd, [1, 3]);
}
