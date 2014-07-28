fn main() {
    use std::default::Default;
    
    let a = (1i, 2i);
    let b = (3i, 4i);
    assert!(a != b);
    
    let c = b.clone();
    assert!(b == c);
    
    let d : (u32, f32) = Default::default();
    assert_eq!(d, (0u32, 0.0f32));
}
