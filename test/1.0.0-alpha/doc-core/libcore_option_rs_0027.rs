fn main() {
    let mut x = Some(2u);
    x.take();
    assert_eq!(x, None);
    
    let mut x: Option<uint> = None;
    x.take();
    assert_eq!(x, None);
}
