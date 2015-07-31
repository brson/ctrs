fn main() {
    let a = (1, 2);
    let b = (3, 4);
    assert!(a != b);
    
    let c = b.clone();
    assert!(b == c);
    
    let d : (u32, f32) = Default::default();
    assert_eq!(d, (0, 0.0f32));
}
