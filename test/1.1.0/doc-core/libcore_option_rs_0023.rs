fn main() {
    fn sq(x: u32) -> Option<u32> { Some(x * x) }
    fn nope(_: u32) -> Option<u32> { None }
    
    assert_eq!(Some(2).and_then(sq).and_then(sq), Some(16));
    assert_eq!(Some(2).and_then(sq).and_then(nope), None);
    assert_eq!(Some(2).and_then(nope).and_then(sq), None);
    assert_eq!(None.and_then(sq).and_then(sq), None);
}
