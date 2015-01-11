fn main() {
    let x = Some(2u);
    let y = None;
    assert_eq!(x.or(y), Some(2u));
    
    let x = None;
    let y = Some(100u);
    assert_eq!(x.or(y), Some(100u));
    
    let x = Some(2u);
    let y = Some(100u);
    assert_eq!(x.or(y), Some(2u));
    
    let x: Option<uint> = None;
    let y = None;
    assert_eq!(x.or(y), None);
}
