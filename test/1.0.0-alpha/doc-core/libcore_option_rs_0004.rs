fn main() {
    let x: Option<uint> = Some(2);
    assert_eq!(x.is_some(), true);
    
    let x: Option<uint> = None;
    assert_eq!(x.is_some(), false);
}
