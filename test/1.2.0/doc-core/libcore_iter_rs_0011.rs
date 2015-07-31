fn main() {
    let xs = [100, 200, 300];
    let mut it = xs.iter().cloned().peekable();
    assert_eq!(*it.peek().unwrap(), 100);
    assert_eq!(it.next().unwrap(), 100);
    assert_eq!(it.next().unwrap(), 200);
    assert_eq!(*it.peek().unwrap(), 300);
    assert_eq!(*it.peek().unwrap(), 300);
    assert_eq!(it.next().unwrap(), 300);
    assert!(it.peek().is_none());
    assert!(it.next().is_none());
}
