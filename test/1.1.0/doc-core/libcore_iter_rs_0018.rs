fn main() {
    fn process<U: Iterator<Item=i32>>(it: U) -> i32 {
    let mut it = it.fuse();
    let mut sum = 0;
    for x in it.by_ref() {
    if x > 5 {
    break;
    }
    sum += x;
    }
    // did we exhaust the iterator?
    if it.next().is_none() {
    sum += 1000;
    }
    sum
    }
    let x = vec![1, 2, 3, 7, 8, 9];
    assert_eq!(process(x.into_iter()), 6);
    let x = vec![1, 2, 3];
    assert_eq!(process(x.into_iter()), 1006);
}
