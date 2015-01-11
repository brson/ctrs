fn main() {
    let a = [1i, 2];
    let mut it = a.iter().filter_map(|&x| if x > 1 {Some(2 * x)} else {None});
    assert_eq!(it.next().unwrap(), 4);
    assert!(it.next().is_none());
}
