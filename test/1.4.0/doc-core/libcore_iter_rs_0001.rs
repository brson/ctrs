fn main() {
    let it = (0..10).filter(|x| x % 2 == 0).chain(15..20);
    assert_eq!((5, Some(15)), it.size_hint());
}
