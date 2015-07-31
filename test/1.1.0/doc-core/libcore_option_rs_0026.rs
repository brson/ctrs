fn main() {
    let mut x = Some(2);
    x.take();
    assert_eq!(x, None);
    
    let mut x: Option<u32> = None;
    x.take();
    assert_eq!(x, None);
}
