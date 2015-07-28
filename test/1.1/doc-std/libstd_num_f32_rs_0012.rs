fn main() {
    use std::f32;
    
    let f = 3.5_f32;
    
    assert_eq!(f.signum(), 1.0);
    assert_eq!(f32::NEG_INFINITY.signum(), -1.0);
    
    assert!(f32::NAN.signum().is_nan());
}
