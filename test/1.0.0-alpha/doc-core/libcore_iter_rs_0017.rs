fn main() {
    fn process<U: Iterator<Item=int>>(it: U) -> int {
        let mut it = it.fuse();
        let mut sum = 0;
        for x in it {
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
    let x = vec![1i,2,3,7,8,9];
    assert_eq!(process(x.into_iter()), 6);
    let x = vec![1i,2,3];
    assert_eq!(process(x.into_iter()), 1006);
}
