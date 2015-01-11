fn main() {
    let x = Some(4u);
    assert_eq!(x.iter().next(), Some(&4));
    
    let x: Option<uint> = None;
    assert_eq!(x.iter().next(), None);
}
